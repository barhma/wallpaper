//! UI orchestration for the wallpaper manager.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use eframe::CreationContext;
use eframe::egui::{
    self, Button, Color32, FontData, FontDefinitions, FontFamily, RichText, Stroke,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetWindowLongW, LWA_ALPHA, SW_RESTORE, SetForegroundWindow,
    SetLayeredWindowAttributes, SetWindowLongW, ShowWindow, WS_EX_LAYERED,
};

use crate::i18n::{Language, Strings, strings};
use crate::image_ops::{
    FolderSource, cached_wallpaper_path, collect_images, pick_random, process_image, stitch_images,
};
use crate::settings::{self, AppSettings, StitchOrientation, ThemeMode};
use crate::slideshow::{SlideshowEvent, SlideshowWorker};
use crate::startup;
use crate::state::AppState;
use crate::theme::apply_theme;
use crate::wallpaper::{StyleMode, set_wallpaper, set_wallpaper_style};

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
    /// Cached HWND for window opacity adjustments.
    window_hwnd: Option<HWND>,
    /// Flag set by the tray event thread when restore is requested.
    tray_restore_requested: Arc<AtomicBool>,
    /// Defer minimizing to tray until after the first frame is shown.
    minimize_pending: bool,
    /// Defer opacity application until the window is fully ready.
    /// Counts down from 2 to 0; opacity is applied when it reaches 0.
    opacity_defer_frames: u8,
    /// Cached image list reused by idle actions and slideshow startup.
    indexed_images: Vec<PathBuf>,
    /// Per-folder image counts derived from the cached image list.
    folder_image_counts: Vec<usize>,
    /// True when the cached image list must be rebuilt from the current sources.
    index_dirty: bool,
}

impl WallpaperApp {
    /// Build the app from persisted settings and OS startup state.
    pub fn new(cc: &CreationContext<'_>, started_from_startup: bool) -> Self {
        configure_fonts(&cc.egui_ctx);
        let mut settings = settings::load();
        settings.window_opacity = settings.window_opacity.clamp(0.98, 1.0);
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
            thread::spawn(move || {
                loop {
                    let Ok(event) = TrayIconEvent::receiver().recv() else {
                        break;
                    };
                    if matches!(
                        event,
                        TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. }
                    ) {
                        if let Some(hwnd) = window_hwnd {
                            unsafe {
                                let _ = ShowWindow(hwnd, SW_RESTORE);
                                let _ = SetForegroundWindow(hwnd);
                            }
                        }
                        restore_flag.store(true, Ordering::SeqCst);
                    }
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
            window_hwnd,
            tray_restore_requested,
            minimize_pending,
            opacity_defer_frames: 2, // Defer opacity for 2 frames so the window is fully ready
            indexed_images: Vec::new(),
            folder_image_counts: Vec::new(),
            index_dirty: true,
        };

        // Don't apply opacity here - defer to first frame for window to be ready

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

        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .exact_height(24.0)
            .show(ctx, |ui| {
                self.render_status_bar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            section_frame(ui, loc(self.state.language, "Actions", "操作"), |ui| {
                self.render_controls(ui, &t)
            });
            ui.add_space(6.0);

            let wide_layout = ui.available_width() >= 860.0;
            if wide_layout {
                ui.columns(2, |columns| {
                    section_frame(
                        &mut columns[0],
                        loc(self.state.language, "Sources", "來源"),
                        |ui| {
                            self.render_sources(ui, &t, &mut settings_changed, &mut restart_needed);
                        },
                    );
                    section_frame(
                        &mut columns[1],
                        loc(self.state.language, "Settings", "設定"),
                        |ui| {
                            ui.label(
                                RichText::new(loc(self.state.language, "Slideshow", "幻燈片"))
                                    .strong(),
                            );
                            self.render_slideshow_options(
                                ui,
                                &t,
                                &mut settings_changed,
                                &mut restart_needed,
                            );
                            ui.separator();
                            ui.label(
                                RichText::new(loc(self.state.language, "Appearance", "外觀"))
                                    .strong(),
                            );
                            self.render_style_selector(
                                ui,
                                &t,
                                &mut settings_changed,
                                &mut restart_needed,
                            );
                            ui.separator();
                            ui.label(RichText::new(t.startup).strong());
                            self.render_startup_section(ui, &t);
                        },
                    );
                });
            } else {
                section_frame(ui, loc(self.state.language, "Sources", "來源"), |ui| {
                    self.render_sources(ui, &t, &mut settings_changed, &mut restart_needed);
                });
                ui.add_space(6.0);
                section_frame(ui, loc(self.state.language, "Settings", "設定"), |ui| {
                    ui.label(
                        RichText::new(loc(self.state.language, "Slideshow", "幻燈片")).strong(),
                    );
                    self.render_slideshow_options(
                        ui,
                        &t,
                        &mut settings_changed,
                        &mut restart_needed,
                    );
                    ui.separator();
                    ui.label(
                        RichText::new(loc(self.state.language, "Appearance", "外觀")).strong(),
                    );
                    self.render_style_selector(ui, &t, &mut settings_changed, &mut restart_needed);
                    ui.separator();
                    ui.label(RichText::new(t.startup).strong());
                    self.render_startup_section(ui, &t);
                });
            }
        });

        if settings_changed {
            if restart_needed {
                self.restart_slideshow_if_running();
            }
            self.persist_settings();
        }
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        let is_error = self.status.contains("failed")
            || self.status.contains("error")
            || self.status.contains("Error");
        let status_color = if is_error {
            Color32::from_rgb(200, 96, 96)
        } else if self.state.running {
            Color32::from_rgb(100, 180, 120)
        } else {
            ui.visuals().text_color()
        };

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(loc(self.state.language, "Status", "狀態")).strong());
            ui.label(RichText::new(&self.status).color(status_color));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.state.running {
                    ui.label(
                        RichText::new(loc(self.state.language, "Running", "執行中"))
                            .strong()
                            .color(status_color),
                    );
                }
            });
        });
    }

    /// Render the language and theme selectors.
    fn render_top_bar(&mut self, ctx: &egui::Context, t: &Strings) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(t.title).strong().size(16.0));
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
                ui.add_space(6.0);
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
                            .selectable_value(
                                &mut self.state.theme,
                                ThemeMode::Light,
                                t.theme_light,
                            )
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
                ui.add_space(6.0);
                ui.label(t.opacity);
                if ui
                    .add(
                        egui::Slider::new(&mut self.state.window_opacity, 0.98..=1.0)
                            .clamp_to_range(true)
                            .show_value(false),
                    )
                    .changed()
                {
                    apply_window_opacity(self.window_hwnd, self.state.window_opacity);
                    self.persist_settings();
                }
                ui.label(format!("{:.0}%", self.state.window_opacity * 100.0));
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
        ui.horizontal_wrapped(|ui| {
            if ui.button(t.add_folder).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.state.folders.push(FolderSource {
                        path,
                        include_subfolders: true,
                    });
                    self.mark_index_dirty();
                    *settings_changed = true;
                    *restart_needed = true;
                }
            }
            if ui.button(t.add_folders).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                    if !paths.is_empty() {
                        self.mark_index_dirty();
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
                    .add_filter(
                        "Images",
                        &["png", "jpg", "jpeg", "bmp", "gif", "tif", "tiff", "webp"],
                    )
                    .pick_file()
                {
                    self.state.single_image = Some(path);
                    self.mark_index_dirty();
                    *settings_changed = true;
                    *restart_needed = true;
                }
            }
            if ui.button(t.clear_all).clicked() {
                self.state.folders.clear();
                self.state.single_image = None;
                self.mark_index_dirty();
                *settings_changed = true;
                *restart_needed = true;
            }
            if ui
                .add_enabled(
                    !self.state.folders.is_empty() || self.state.single_image.is_some(),
                    Button::new(loc(self.state.language, "Refresh index", "重新整理索引")),
                )
                .clicked()
            {
                if let Err(err) = self.ensure_image_index() {
                    self.status = err.to_string();
                } else {
                    self.status = format!(
                        "{}: {}",
                        loc(self.state.language, "Indexed images", "索引圖片"),
                        self.indexed_images.len()
                    );
                }
            }
        });

        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(format!(
                "{}: {}",
                loc(self.state.language, "Folders", "資料夾"),
                self.state.folders.len()
            ));
            ui.separator();
            ui.label(format!(
                "{}: {}",
                loc(self.state.language, "Single image", "單張圖片"),
                if self.state.single_image.is_some() {
                    loc(self.state.language, "Selected", "已選取")
                } else {
                    loc(self.state.language, "None", "無")
                }
            ));
            ui.separator();
            ui.label(format!(
                "{}: {}",
                loc(self.state.language, "Image index", "圖片索引"),
                if self.index_dirty {
                    loc(self.state.language, "Pending refresh", "待重新整理").to_string()
                } else {
                    self.indexed_images.len().to_string()
                }
            ));
        });
        ui.add_space(6.0);

        egui::ScrollArea::vertical()
            .id_source("folder_list")
            .max_height(360.0)
            .show(ui, |ui| {
                if self.state.folders.is_empty() && self.state.single_image.is_none() {
                    ui.label(loc(
                        self.state.language,
                        "No sources added yet.",
                        "尚未加入任何來源。",
                    ));
                }

                let mut to_remove = Vec::new();
                let mut source_flags_changed = false;
                for (idx, folder) in self.state.folders.iter_mut().enumerate() {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(display_name(folder.path.as_path())).strong());
                        if let Some(count) = self.folder_image_counts.get(idx) {
                            if !self.index_dirty {
                                ui.label(
                                    RichText::new(format!(
                                        "{}: {}",
                                        loc(self.state.language, "Images", "圖片"),
                                        count
                                    ))
                                    .weak(),
                                );
                            }
                        }
                        if ui.button(t.remove).clicked() {
                            to_remove.push(idx);
                        }
                    });
                    ui.label(
                        RichText::new(folder.path.display().to_string())
                            .small()
                            .weak(),
                    );
                    if ui
                        .checkbox(&mut folder.include_subfolders, t.include_subfolders)
                        .changed()
                    {
                        source_flags_changed = true;
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    ui.separator();
                }
                if source_flags_changed {
                    self.mark_index_dirty();
                }
                for idx in to_remove.into_iter().rev() {
                    self.state.folders.remove(idx);
                    self.mark_index_dirty();
                    *settings_changed = true;
                    *restart_needed = true;
                }

                let mut clear_single = false;
                if let Some(img) = self.state.single_image.as_ref() {
                    ui.label(
                        RichText::new(loc(self.state.language, "Single image", "單張圖片"))
                            .strong(),
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(img.display().to_string()).small().weak());
                        if ui.button(t.remove).clicked() {
                            clear_single = true;
                        }
                    });
                }
                if clear_single {
                    self.state.single_image = None;
                    self.mark_index_dirty();
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
        ui.horizontal_wrapped(|ui| {
            if ui
                .checkbox(&mut self.state.auto_rotate, t.auto_rotate)
                .changed()
            {
                *settings_changed = true;
                *restart_needed = true;
            }
            if ui
                .checkbox(&mut self.state.random_order, t.random_order)
                .changed()
            {
                *settings_changed = true;
                *restart_needed = true;
            }
        });

        ui.add_space(6.0);
        ui.label(RichText::new(t.interval_seconds).strong());
        ui.horizontal_wrapped(|ui| {
            for (secs, label) in [(10_u64, "10s"), (30, "30s"), (60, "1m"), (300, "5m")] {
                let selected = self.state.interval_secs == secs;
                if ui
                    .add(
                        Button::new(label)
                            .fill(if selected {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().faint_bg_color
                            })
                            .stroke(Stroke::NONE),
                    )
                    .clicked()
                {
                    self.state.interval_secs = secs;
                    *settings_changed = true;
                    *restart_needed = true;
                }
            }
        });

        ui.horizontal(|ui| {
            if ui
                .add(
                    egui::Slider::new(&mut self.state.interval_secs, 5..=7200)
                        .text(t.interval_seconds),
                )
                .changed()
            {
                *settings_changed = true;
                *restart_needed = true;
            }
        });

        ui.add_space(6.0);
        if ui
            .checkbox(&mut self.state.stitch_enabled, t.stitch_enabled)
            .changed()
        {
            *settings_changed = true;
            *restart_needed = true;
        }

        if self.state.stitch_enabled {
            egui::Grid::new("stitch_grid")
                .num_columns(2)
                .spacing(egui::vec2(8.0, 6.0))
                .show(ui, |ui| {
                    ui.label(t.stitch_count);
                    let mut count = self.state.stitch_count as i32;
                    if ui.add(egui::Slider::new(&mut count, 2..=5)).changed() {
                        self.state.stitch_count = count as u8;
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    ui.end_row();

                    ui.label(t.stitch_orientation);
                    let orientation_text = match self.state.stitch_orientation {
                        StitchOrientation::Horizontal => t.stitch_horizontal,
                        StitchOrientation::Vertical => t.stitch_vertical,
                    };
                    egui::ComboBox::from_id_source("stitch_orientation_combo")
                        .selected_text(orientation_text)
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.state.stitch_orientation,
                                    StitchOrientation::Horizontal,
                                    t.stitch_horizontal,
                                )
                                .changed()
                            {
                                *settings_changed = true;
                                *restart_needed = true;
                            }
                            if ui
                                .selectable_value(
                                    &mut self.state.stitch_orientation,
                                    StitchOrientation::Vertical,
                                    t.stitch_vertical,
                                )
                                .changed()
                            {
                                *settings_changed = true;
                                *restart_needed = true;
                            }
                        });
                    ui.end_row();

                    ui.label(t.stitch_crop_width);
                    if ui
                        .add(egui::Slider::new(
                            &mut self.state.stitch_crop_width,
                            640..=7680,
                        ))
                        .changed()
                    {
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    ui.end_row();

                    ui.label(t.stitch_crop_height);
                    if ui
                        .add(egui::Slider::new(
                            &mut self.state.stitch_crop_height,
                            480..=4320,
                        ))
                        .changed()
                    {
                        *settings_changed = true;
                        *restart_needed = true;
                    }
                    ui.end_row();
                });
        }
    }

    /// Render startup-related options (run on startup, minimize to tray).
    fn render_startup_section(&mut self, ui: &mut egui::Ui, t: &Strings) {
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
                        .selectable_value(
                            &mut self.state.style,
                            mode,
                            style_label(mode, self.state.language),
                        )
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

    /// Render action buttons (apply once, next, start/stop, reset).
    fn render_controls(&mut self, ui: &mut egui::Ui, t: &Strings) {
        let primary_fill = if self.state.running {
            Color32::from_rgb(186, 70, 70)
        } else {
            Color32::from_rgb(61, 140, 100)
        };
        let primary_label = if self.state.running { t.stop } else { t.start };
        ui.horizontal_wrapped(|ui| {
            if ui.button(t.apply_once).clicked() {
                match self.apply_once() {
                    Ok(_) => self.status = format!("{} ({})", t.apply_once, t.status_idle),
                    Err(err) => self.status = err.to_string(),
                }
            }

            if ui.button(t.next_image).clicked() {
                self.request_next();
            }

            if ui
                .add(
                    Button::new(RichText::new(primary_label).strong())
                        .fill(primary_fill)
                        .stroke(Stroke::NONE),
                )
                .clicked()
            {
                if self.state.running {
                    self.stop_worker();
                    self.status = t.status_idle.to_string();
                    self.persist_settings();
                } else {
                    match self.start_slideshow() {
                        Ok(_) => {
                            self.state.running = true;
                            self.status = t.status_running.to_string();
                            self.persist_settings();
                        }
                        Err(err) => self.status = err.to_string(),
                    }
                }
            }

            if ui.button(t.reset_defaults).clicked() {
                self.reset_to_defaults(ui.ctx());
            }
        });
    }

    /// Reset all settings to defaults.
    fn reset_to_defaults(&mut self, ctx: &egui::Context) {
        self.stop_worker();

        let defaults = AppSettings::default();
        self.settings = defaults.clone();
        self.state = AppState::from_settings(&defaults);
        self.mark_index_dirty();

        // Apply theme and opacity
        apply_theme(ctx, self.state.theme);
        apply_window_opacity(self.window_hwnd, self.state.window_opacity);

        // Persist and update status
        let _ = settings::save(&self.settings);
        let t = strings(self.state.language);
        self.status = t.status_idle.to_string();
    }

    /// Apply a single wallpaper immediately, without starting the slideshow.
    fn apply_once(&mut self) -> Result<()> {
        self.ensure_image_index()?;
        let images = self.indexed_images.clone();
        if images.is_empty() {
            let t = strings(self.state.language);
            return Err(anyhow::anyhow!(t.no_images));
        }
        set_wallpaper_style(self.state.style)?;

        let processed = if self.state.stitch_enabled {
            let count = (self.state.stitch_count as usize).min(images.len());
            let mut selected = Vec::with_capacity(count);
            let mut last: Option<std::path::PathBuf> = None;
            for _ in 0..count {
                let choice = pick_random(&images, last.as_ref())?;
                last = Some(choice.clone());
                selected.push(choice);
            }
            stitch_images(
                &selected,
                self.state.auto_rotate,
                self.state.stitch_orientation,
                true, // always crop
                self.state.stitch_crop_width,
                self.state.stitch_crop_height,
            )?
        } else {
            let choice = pick_random(&images, None)?;
            process_image(&choice, self.state.auto_rotate)?
        };

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
        self.ensure_image_index()?;
        if self.indexed_images.is_empty() {
            let t = strings(self.state.language);
            return Err(anyhow::anyhow!(t.no_images));
        }
        let worker = SlideshowWorker::start(
            self.indexed_images.clone(),
            self.state.auto_rotate,
            self.state.style,
            Duration::from_secs(self.state.interval_secs),
            self.state.random_order,
            self.state.stitch_enabled,
            self.state.stitch_count,
            self.state.stitch_orientation,
            self.state.stitch_crop_width,
            self.state.stitch_crop_height,
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

    /// Mark the in-memory image index as stale after source changes.
    fn mark_index_dirty(&mut self) {
        self.index_dirty = true;
        self.indexed_images.clear();
        self.folder_image_counts.clear();
    }

    /// Ensure a current in-memory image index is available before applying wallpapers.
    fn ensure_image_index(&mut self) -> Result<()> {
        if !self.index_dirty {
            return Ok(());
        }

        let images = collect_images(&self.state.folders, self.state.single_image.as_deref())?;
        self.folder_image_counts = self
            .state
            .folders
            .iter()
            .map(|folder| {
                images
                    .iter()
                    .filter(|path| source_contains(folder, path.as_path()))
                    .count()
            })
            .collect();
        self.indexed_images = images;
        self.index_dirty = false;
        Ok(())
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
        // Apply deferred opacity once the window is fully ready (after 2 frames).
        if self.opacity_defer_frames > 1 {
            self.opacity_defer_frames -= 1;
            ctx.request_repaint();
        } else if self.opacity_defer_frames == 1 {
            self.opacity_defer_frames = 0;
            apply_window_opacity(self.window_hwnd, self.state.window_opacity);
        }

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
            // Re-apply opacity after restore; ShowWindow(SW_RESTORE) can strip WS_EX_LAYERED.
            apply_window_opacity(self.window_hwnd, self.state.window_opacity);
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

/// Apply a per-window opacity using Win32 layered window attributes.
fn apply_window_opacity(hwnd: Option<HWND>, opacity: f32) {
    let Some(hwnd) = hwnd else {
        return;
    };
    let clamped = opacity.clamp(0.98, 1.0);
    let alpha = (clamped * 255.0).round() as u8;
    unsafe {
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, style | WS_EX_LAYERED.0 as i32);
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
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
            let (r, g, b) = if border {
                (255, 255, 255)
            } else {
                (60, 120, 220)
            };
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

fn loc(lang: Language, en: &'static str, cht: &'static str) -> &'static str {
    match lang {
        Language::En => en,
        Language::Cht => cht,
    }
}

fn display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

fn source_contains(source: &FolderSource, path: &Path) -> bool {
    if source.include_subfolders {
        path.starts_with(&source.path)
    } else {
        path.parent()
            .map(|parent| parent == source.path.as_path())
            .unwrap_or(false)
    }
}

fn section_frame<R>(
    ui: &mut egui::Ui,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    ui.group(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(4.0);
        add_contents(ui)
    })
    .inner
}
