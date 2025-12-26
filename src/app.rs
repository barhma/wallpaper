//! UI orchestration for the wallpaper manager.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, RichText};
use eframe::CreationContext;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{SetForegroundWindow, ShowWindow, SW_RESTORE};

use crate::i18n::{strings, Language, Strings};
use crate::image_ops::{cached_wallpaper_path, collect_images, pick_random, process_image, FolderSource};
use crate::settings::{self, AppSettings, ThemeMode};
use crate::slideshow::{SlideshowEvent, SlideshowWorker};
use crate::startup;
use crate::state::AppState;
use crate::theme::apply_theme;
use crate::wallpaper::{set_wallpaper, set_wallpaper_style, StyleMode};

/// Main application container that owns UI state and background workers.
pub struct WallpaperApp {
    /// Runtime settings that drive UI and slideshow behavior.
    state: AppState,
    /// Persisted settings stored on disk.
    settings: AppSettings,
    /// Last status text shown to the user.
    status: String,
    /// Active slideshow worker, if running.
    worker: Option<SlideshowWorker>,
    /// Tray icon handle.
    tray_icon: Option<TrayIcon>,
    /// Flag set by the tray event thread when restore is requested.
    tray_restore_requested: Arc<AtomicBool>,
    /// Defer minimizing to tray until after the first frame is shown.
    minimize_pending: bool,
}

impl WallpaperApp {
    /// Build the app from persisted settings and OS startup state.
    pub fn new(cc: &CreationContext<'_>, started_from_startup: bool) -> Self {
        configure_fonts(&cc.egui_ctx);
        let mut settings = settings::load();
        if let Ok(enabled) = startup::is_enabled() {
            if settings.run_on_startup != enabled {
                settings.run_on_startup = enabled;
                let _ = settings::save(&settings);
            }
        }

        let state = AppState::from_settings(&settings);
        let status = strings(state.language).status_idle.to_string();
        apply_theme(&cc.egui_ctx, state.theme);

        let window_hwnd = window_hwnd_from_context(cc);
        let tray_icon = create_tray_icon(state.language);
        let tray_restore_requested = Arc::new(AtomicBool::new(false));
        if tray_icon.is_some() {
            let restore_flag = Arc::clone(&tray_restore_requested);
            thread::spawn(move || loop {
                let Ok(event) = TrayIconEvent::receiver().recv() else {
                    break;
                };
                if matches!(event, TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. }) {
                    if let Some(hwnd) = window_hwnd {
                        unsafe {
                            let _ = ShowWindow(hwnd, SW_RESTORE);
                            let _ = SetForegroundWindow(hwnd);
                        }
                    }
                    restore_flag.store(true, Ordering::SeqCst);
                }
            });
        }

        let minimize_pending = settings.minimize_to_tray_on_start && started_from_startup;
        let should_start = state.running;

        let mut app = Self {
            state,
            status,
            worker: None,
            settings,
            tray_icon,
            tray_restore_requested,
            minimize_pending,
        };

        if should_start {
            if let Err(err) = app.start_slideshow() {
                app.status = err.to_string();
                app.state.running = false;
                app.settings.running = false;
                let _ = settings::save(&app.settings);
            }
        }

        app
    }

    /// Render the application UI and react to user input.
    pub fn ui(&mut self, ctx: &egui::Context) {
        self.drain_events();
        self.handle_tray_events(ctx);

        let t = strings(self.state.language);
        self.render_top_bar(ctx, &t);

        let mut settings_changed = false;
        let mut restart_needed = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_sources(ui, &t, &mut settings_changed, &mut restart_needed);
            self.render_slideshow_options(ui, &t, &mut settings_changed, &mut restart_needed);
            self.render_startup_section(ui, &t);
            self.render_style_selector(ui, &t, &mut settings_changed, &mut restart_needed);
            self.render_controls(ui, &t);

            ui.separator();
            ui.label(format!("Status: {}", self.status));
        });

        if settings_changed {
            if restart_needed {
                self.restart_slideshow_if_running();
            }
            self.persist_settings();
        }
    }

    /// Render the language and theme selectors.
    fn render_top_bar(&mut self, ctx: &egui::Context, t: &Strings) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading(RichText::new(t.title).strong());
            ui.horizontal(|ui| {
                ui.label(t.language);
                egui::ComboBox::from_id_source("language_combo")
                    .selected_text(match self.state.language {
                        Language::En => "English",
                        Language::Cht => "繁體中文",
                    })
                    .show_ui(ui, |ui| {
                        let mut changed = false;
                        if ui
                            .selectable_value(&mut self.state.language, Language::En, "English")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.state.language, Language::Cht, "繁體中文")
                            .changed()
                        {
                            changed = true;
                        }
                        if changed {
                            self.persist_settings();
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label(t.theme);
                let theme_text = match self.state.theme {
                    ThemeMode::Light => t.theme_light,
                    ThemeMode::Dark => t.theme_dark,
                };
                let mut changed = false;
                egui::ComboBox::from_id_source("theme_combo")
                    .selected_text(theme_text)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(&mut self.state.theme, ThemeMode::Light, t.theme_light)
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.state.theme, ThemeMode::Dark, t.theme_dark)
                            .changed()
                        {
                            changed = true;
                        }
                    });
                if changed {
                    apply_theme(ctx, self.state.theme);
                    self.persist_settings();
                }
            });
        });
    }

    /// Render folder and single-image selection UI.
    fn render_sources(
        &mut self,
        ui: &mut egui::Ui,
        t: &Strings,
        settings_changed: &mut bool,
        restart_needed: &mut bool,
    ) {
        ui.horizontal(|ui| {
            if ui.button(t.add_folder).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.state.folders.push(FolderSource {
                        path,
                        include_subfolders: true,
                    });
                    *settings_changed = true;
                    *restart_needed = true;
                }
            }
            if ui.button(t.add_folders).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                    if !paths.is_empty() {
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    for path in paths {
                        self.state.folders.push(FolderSource {
                            path,
                            include_subfolders: true,
                        });
                    }
                }
            }
            if ui.button(t.add_image).clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &[
                        "png", "jpg", "jpeg", "bmp", "gif", "tif", "tiff", "webp",
                    ])
                    .pick_file()
                {
                    self.state.single_image = Some(path);
                    *settings_changed = true;
                    *restart_needed = true;
                }
            }
            if ui.button(t.clear_all).clicked() {
                self.state.folders.clear();
                self.state.single_image = None;
                *settings_changed = true;
                *restart_needed = true;
            }
        });

        ui.separator();

        egui::ScrollArea::vertical().id_source("folder_list").show(ui, |ui| {
            let mut to_remove = Vec::new();
            for (idx, folder) in self.state.folders.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(folder.path.display().to_string());
                    if ui
                        .checkbox(&mut folder.include_subfolders, t.include_subfolders)
                        .changed()
                    {
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    if ui.button(t.remove).clicked() {
                        to_remove.push(idx);
                    }
                });
            }
            for idx in to_remove.into_iter().rev() {
                self.state.folders.remove(idx);
                *settings_changed = true;
                *restart_needed = true;
            }

            let mut clear_single = false;
            if let Some(img) = self.state.single_image.as_ref() {
                let label = format!("Single: {}", img.display());
                ui.horizontal(|ui| {
                    ui.label(label);
                    if ui.button(t.remove).clicked() {
                        clear_single = true;
                    }
                });
            }
            if clear_single {
                self.state.single_image = None;
                *settings_changed = true;
                *restart_needed = true;
            }
        });
    }

    /// Render slideshow toggles and interval slider.
    fn render_slideshow_options(
        &mut self,
        ui: &mut egui::Ui,
        t: &Strings,
        settings_changed: &mut bool,
        restart_needed: &mut bool,
    ) {
        ui.separator();

        ui.horizontal(|ui| {
            if ui.checkbox(&mut self.state.auto_rotate, t.auto_rotate).changed() {
                *settings_changed = true;
                *restart_needed = true;
            }
            if ui.checkbox(&mut self.state.random_order, t.random_order).changed() {
                *settings_changed = true;
                *restart_needed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label(t.slideshow);
            if ui
                .add(egui::Slider::new(&mut self.state.interval_secs, 5..=7200).text(t.interval_seconds))
                .changed()
            {
                *settings_changed = true;
                *restart_needed = true;
            }
        });
    }

    /// Render startup-related options (run on startup, minimize to tray).
    fn render_startup_section(&mut self, ui: &mut egui::Ui, t: &Strings) {
        ui.separator();
        ui.label(t.startup);
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.settings.run_on_startup, t.run_on_startup)
                .changed()
            {
                let result = if self.settings.run_on_startup {
                    startup::enable()
                } else {
                    startup::disable()
                };
                if let Err(err) = result {
                    self.status = err.to_string();
                    self.settings.run_on_startup = !self.settings.run_on_startup;
                } else if let Err(err) = settings::save(&self.settings) {
                    self.status = err.to_string();
                }
            }
            if ui
                .checkbox(
                    &mut self.settings.minimize_to_tray_on_start,
                    t.minimize_to_tray_on_start,
                )
                .changed()
            {
                if let Err(err) = settings::save(&self.settings) {
                    self.status = err.to_string();
                }
            }
        });
        if ui.button(t.minimize_to_tray).clicked() {
            self.minimize_to_tray(ui.ctx());
        }
    }

    /// Render the wallpaper style selection and apply it immediately.
    fn render_style_selector(
        &mut self,
        ui: &mut egui::Ui,
        t: &Strings,
        settings_changed: &mut bool,
        restart_needed: &mut bool,
    ) {
        let mut style_changed = false;
        egui::ComboBox::from_label(t.style)
            .selected_text(style_label(self.state.style, self.state.language))
            .show_ui(ui, |ui| {
                for mode in StyleMode::ALL {
                    if ui
                        .selectable_value(&mut self.state.style, mode, style_label(mode, self.state.language))
                        .changed()
                    {
                        style_changed = true;
                    }
                }
            });
        if style_changed {
            *settings_changed = true;
            *restart_needed = true;
            if let Err(err) = set_wallpaper_style(self.state.style) {
                self.status = err.to_string();
            } else if !self.state.running {
                // Reapply the cached wallpaper so the new style takes effect immediately.
                if let Ok(cache_path) = cached_wallpaper_path() {
                    if cache_path.exists() {
                        let _ = set_wallpaper(&cache_path);
                    }
                }
            }
        }
    }

    /// Render action buttons (apply once, next, start/stop).
    fn render_controls(&mut self, ui: &mut egui::Ui, t: &Strings) {
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button(t.apply_once).clicked() {
                match self.apply_once() {
                    Ok(_) => self.status = format!("{} ({})", t.apply_once, t.status_idle),
                    Err(err) => self.status = err.to_string(),
                }
            }

            if ui.button(t.next_image).clicked() {
                self.request_next();
            }

            if self.state.running {
                if ui.button(t.stop).clicked() {
                    self.stop_worker();
                    self.status = t.status_idle.to_string();
                    self.persist_settings();
                }
            } else if ui.button(t.start).clicked() {
                match self.start_slideshow() {
                    Ok(_) => {
                        self.state.running = true;
                        self.status = t.status_running.to_string();
                        self.persist_settings();
                    }
                    Err(err) => self.status = err.to_string(),
                }
            }
        });
    }

    /// Apply a single wallpaper immediately, without starting the slideshow.
    fn apply_once(&mut self) -> Result<()> {
        let images = collect_images(&self.state.folders, self.state.single_image.as_deref())?;
        if images.is_empty() {
            let t = strings(self.state.language);
            return Err(anyhow::anyhow!(t.no_images));
        }
        set_wallpaper_style(self.state.style)?;
        let choice = pick_random(&images, None)?;
        let processed = process_image(&choice, self.state.auto_rotate)?;
        set_wallpaper(&processed)?;
        Ok(())
    }

    /// Advance the slideshow or apply a single image when idle.
    fn request_next(&mut self) {
        if let Some(worker) = &self.worker {
            worker.request_next();
        } else {
            match self.apply_once() {
                Ok(_) => {
                    let t = strings(self.state.language);
                    self.status = format!("{} ({})", t.next_image, t.status_idle);
                }
                Err(err) => self.status = err.to_string(),
            }
        }
    }

    /// Start (or restart) the slideshow worker.
    fn start_slideshow(&mut self) -> Result<()> {
        self.stop_worker();
        let images = collect_images(&self.state.folders, self.state.single_image.as_deref())?;
        if images.is_empty() {
            let t = strings(self.state.language);
            return Err(anyhow::anyhow!(t.no_images));
        }

        let worker = SlideshowWorker::start(
            images,
            self.state.auto_rotate,
            self.state.style,
            Duration::from_secs(self.state.interval_secs),
            self.state.random_order,
        )?;

        self.worker = Some(worker);
        self.state.running = true;
        Ok(())
    }

    /// Stop the slideshow worker without blocking the UI thread.
    fn stop_worker(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.stop();
        }
        self.state.running = false;
    }

    /// Restart the slideshow if it is currently running.
    fn restart_slideshow_if_running(&mut self) {
        if !self.state.running {
            return;
        }
        match self.start_slideshow() {
            Ok(_) => {
                let t = strings(self.state.language);
                self.status = t.status_running.to_string();
            }
            Err(err) => {
                self.status = err.to_string();
                self.state.running = false;
            }
        }
    }

    /// Persist the current runtime state into the settings file.
    fn persist_settings(&mut self) {
        self.state.apply_to_settings(&mut self.settings);
        if let Err(err) = settings::save(&self.settings) {
            self.status = err.to_string();
        }
    }

    /// Drain background worker events into UI state.
    fn drain_events(&mut self) {
        let mut events = Vec::new();
        if let Some(worker) = &self.worker {
            worker.drain_events(&mut events);
        }
        for evt in events {
            match evt {
                SlideshowEvent::Info(msg) => self.status = msg,
                SlideshowEvent::Error(msg) => {
                    self.status = msg;
                    self.state.running = false;
                    self.persist_settings();
                }
            }
        }
    }

    /// Handle minimize and restore events from the tray icon.
    fn handle_tray_events(&mut self, ctx: &egui::Context) {
        if let Some(minimized) = ctx.input(|i| i.viewport().minimized) {
            if minimized {
                self.minimize_to_tray(ctx);
            }
        }

        if self.minimize_pending {
            self.minimize_pending = false;
            self.minimize_to_tray(ctx);
        }

        if self.tray_restore_requested.swap(false, Ordering::SeqCst) {
            self.restore_from_tray(ctx);
        }
    }

    /// Hide the window and show the tray icon.
    fn minimize_to_tray(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        if let Some(tray_icon) = &self.tray_icon {
            let _ = tray_icon.set_visible(true);
        }
    }

    /// Restore the window from the tray icon.
    fn restore_from_tray(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        if let Some(tray_icon) = &self.tray_icon {
            let _ = tray_icon.set_visible(false);
        }
    }
}

/// Configure fonts so CJK text renders correctly when available.
fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let candidates = [
        r"C:\Windows\Fonts\NotoSansTC-VF.ttf",
        r"C:\Windows\Fonts\NotoSansSC-VF.ttf",
        r"C:\Windows\Fonts\NotoSansHK-VF.ttf",
        r"C:\Windows\Fonts\GenJyuuGothic-Monospace-Regular.ttf",
        r"C:\Windows\Fonts\GenJyuuGothic-Monospace-Normal.ttf",
        r"C:\Windows\Fonts\GenJyuuGothic-Monospace-Medium.ttf",
        r"C:\Windows\Fonts\GenJyuuGothic-Monospace-Bold.ttf",
        r"C:\Windows\Fonts\simhei.ttf",
        r"C:\Windows\Fonts\simkai.ttf",
        r"C:\Windows\Fonts\simfang.ttf",
        r"C:\Windows\Fonts\simsunb.ttf",
        r"C:\Windows\Fonts\SimsunExtG.ttf",
        r"C:\Windows\Fonts\kaiu.ttf",
        r"C:\Windows\Fonts\arialuni.ttf",
    ];
    let mut font_data = None;
    for path in candidates {
        if let Ok(data) = std::fs::read(path) {
            font_data = Some(data);
            break;
        }
    }
    let Some(data) = font_data else {
        return;
    };
    fonts
        .font_data
        .insert("cjk".to_string(), FontData::from_owned(data));
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "cjk".to_string());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "cjk".to_string());
    ctx.set_fonts(fonts);
}

/// Extract the Win32 HWND for native window operations.
fn window_hwnd_from_context(cc: &CreationContext<'_>) -> Option<HWND> {
    let handle = cc.window_handle().ok()?;
    match handle.as_raw() {
        RawWindowHandle::Win32(win32) => Some(HWND(win32.hwnd.get())),
        _ => None,
    }
}

impl Drop for WallpaperApp {
    /// Ensure the worker thread is stopped before shutdown.
    fn drop(&mut self) {
        self.stop_worker();
    }
}

impl eframe::App for WallpaperApp {
    /// egui frame entry point.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

/// Create a tray icon with a fallback in-memory bitmap.
fn create_tray_icon(language: Language) -> Option<TrayIcon> {
    let icon = default_tray_icon()?;
    let t = strings(language);

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip(t.title)
        .with_icon(icon)
        .build();

    tray_icon.ok().map(|tray| {
        let _ = tray.set_visible(false);
        tray
    })
}

/// Build a simple fallback tray icon (white border, blue fill).
fn default_tray_icon() -> Option<Icon> {
    let size = 16u32;
    let mut rgba = Vec::with_capacity((size * size * 4) as usize);
    for y in 0..size {
        for x in 0..size {
            let border = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let (r, g, b) = if border { (255, 255, 255) } else { (60, 120, 220) };
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    }
    Icon::from_rgba(rgba, size, size).ok()
}

/// Map a style enum to its label, honoring language-specific labels.
fn style_label(mode: StyleMode, lang: Language) -> &'static str {
    match (mode, lang) {
        (StyleMode::Fill, Language::Cht) => "填滿",
        (StyleMode::Fit, Language::Cht) => "置中填入",
        (StyleMode::Stretch, Language::Cht) => "拉伸",
        (StyleMode::Tile, Language::Cht) => "並排",
        (StyleMode::Center, Language::Cht) => "置中",
        (StyleMode::Span, Language::Cht) => "跨螢幕",
        (m, _) => m.label(),
    }
}
