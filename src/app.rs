use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, RichText};
use eframe::CreationContext;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{SetForegroundWindow, ShowWindow, SW_RESTORE};

use crate::i18n::{strings, Language};
use crate::image_ops::{collect_images, pick_random, process_image, FolderSource};
use crate::settings::{self, AppSettings};
use crate::startup;
use crate::wallpaper::{set_wallpaper, set_wallpaper_style, StyleMode};

pub struct WallpaperApp {
    folders: Vec<FolderSource>,
    single_image: Option<PathBuf>,
    auto_rotate: bool,
    random_order: bool,
    interval_secs: u64,
    language: Language,
    style: StyleMode,
    status: String,
    running: bool,
    worker: Option<WorkerHandle>,
    settings: AppSettings,
    tray_icon: Option<TrayIcon>,
    tray_restore_requested: Arc<AtomicBool>,
    minimize_pending: bool,
}

struct WorkerHandle {
    cmd_tx: Sender<WorkerCommand>,
    event_rx: Receiver<WorkerEvent>,
    join: Option<thread::JoinHandle<()>>,
}

enum WorkerCommand {
    Stop,
}

enum WorkerEvent {
    Info(String),
    Error(String),
}

impl WallpaperApp {
    pub fn new(cc: &CreationContext<'_>, started_from_startup: bool) -> Self {
        let language = Language::En;
        configure_fonts(&cc.egui_ctx);
        let status = strings(language).status_idle.to_string();
        let mut settings = settings::load();
        if let Ok(enabled) = startup::is_enabled() {
            settings.run_on_startup = enabled;
        }
        let window_hwnd = window_hwnd_from_context(cc);
        let tray_icon = create_tray_icon(language);
        let tray_restore_requested = Arc::new(AtomicBool::new(false));
        if tray_icon.is_some() {
            let restore_flag = Arc::clone(&tray_restore_requested);
            let hwnd = window_hwnd;
            thread::spawn(move || loop {
                let Ok(event) = TrayIconEvent::receiver().recv() else {
                    break;
                };
                if matches!(event, TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. }) {
                    if let Some(hwnd) = hwnd {
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
        Self {
            folders: Vec::new(),
            single_image: None,
            auto_rotate: true,
            random_order: true,
            interval_secs: 600,
            language,
            style: StyleMode::Fill,
            status,
            running: false,
            worker: None,
            settings,
            tray_icon,
            tray_restore_requested,
            minimize_pending,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        self.drain_events();
        self.handle_tray_events(ctx);

        let t = strings(self.language);
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading(RichText::new(t.title).strong());
            ui.horizontal(|ui| {
                ui.label(t.language);
                egui::ComboBox::from_id_source("language_combo")
                    .selected_text(match self.language {
                        Language::En => "English",
                        Language::Cht => "繁體中文",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.language, Language::En, "English");
                        ui.selectable_value(&mut self.language, Language::Cht, "繁體中文");
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(t.add_folder).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.folders.push(FolderSource {
                            path,
                            include_subfolders: true,
                        });
                    }
                }
                if ui.button(t.add_folders).clicked() {
                    if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                        for path in paths {
                            self.folders.push(FolderSource {
                                path,
                                include_subfolders: true,
                            });
                        }
                    }
                }
                if ui.button(t.add_image).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif", "tif", "tiff", "webp"]).pick_file() {
                        self.single_image = Some(path);
                    }
                }
                if ui.button("Clear").clicked() {
                    self.folders.clear();
                    self.single_image = None;
                }
            });

            ui.separator();

            egui::ScrollArea::vertical().id_source("folder_list").show(ui, |ui| {
                let mut to_remove = Vec::new();
                for (idx, folder) in self.folders.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(folder.path.display().to_string());
                        ui.checkbox(&mut folder.include_subfolders, t.include_subfolders);
                        if ui.button("✕").clicked() {
                            to_remove.push(idx);
                        }
                    });
                }
                for idx in to_remove.into_iter().rev() {
                    self.folders.remove(idx);
                }
                let mut clear_single = false;
                if let Some(img) = self.single_image.as_ref() {
                    let label = format!("Single: {}", img.display());
                    ui.horizontal(|ui| {
                        ui.label(label);
                        if ui.button("✕").clicked() {
                            clear_single = true;
                        }
                    });
                }
                if clear_single {
                    self.single_image = None;
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_rotate, t.auto_rotate);
                ui.checkbox(&mut self.random_order, t.random_order);
            });

            ui.horizontal(|ui| {
                ui.label(t.slideshow);
                ui.add(egui::Slider::new(&mut self.interval_secs, 5..=7200).text(t.interval_seconds));
            });

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
                self.minimize_to_tray(ctx);
            }

            egui::ComboBox::from_label(t.style)
                .selected_text(style_label(self.style, self.language))
                .show_ui(ui, |ui| {
                    for mode in StyleMode::ALL {
                        ui.selectable_value(&mut self.style, mode, style_label(mode, self.language));
                    }
                });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button(t.apply_once).clicked() {
                    match self.apply_once() {
                        Ok(_) => self.status = format!("{} ({})", t.apply_once, t.status_idle),
                        Err(err) => self.status = err.to_string(),
                    }
                }

                if self.running {
                    if ui.button(t.stop).clicked() {
                        self.stop_worker();
                        self.status = t.status_idle.to_string();
                    }
                } else if ui.button(t.start).clicked() {
                    match self.start_slideshow() {
                        Ok(_) => {
                            self.running = true;
                            self.status = t.status_running.to_string();
                        }
                        Err(err) => self.status = err.to_string(),
                    }
                }
            });

            ui.separator();
            ui.label(format!("Status: {}", self.status));
        });
    }

    fn apply_once(&mut self) -> Result<()> {
        let images = collect_images(&self.folders, self.single_image.as_deref())?;
        if images.is_empty() {
            let t = strings(self.language);
            return Err(anyhow::anyhow!(t.no_images));
        }
        set_wallpaper_style(self.style)?;
        let choice = pick_random(&images, None)?;
        let processed = process_image(&choice, self.auto_rotate)?;
        set_wallpaper(&processed)?;
        Ok(())
    }

    fn start_slideshow(&mut self) -> Result<()> {
        self.stop_worker();
        let images = collect_images(&self.folders, self.single_image.as_deref())?;
        if images.is_empty() {
            let t = strings(self.language);
            return Err(anyhow::anyhow!(t.no_images));
        }

        set_wallpaper_style(self.style)?;

        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (evt_tx, evt_rx) = mpsc::channel();

        let auto_rotate = self.auto_rotate;
        let style = self.style;
        let interval = Duration::from_secs(self.interval_secs);
        let random_order = self.random_order;

        let handle = thread::spawn(move || {
            let _ = run_worker(
                images,
                auto_rotate,
                style,
                interval,
                random_order,
                cmd_rx,
                evt_tx,
            );
        });

        self.worker = Some(WorkerHandle {
            cmd_tx,
            event_rx: evt_rx,
            join: Some(handle),
        });
        self.running = true;
        Ok(())
    }

    fn stop_worker(&mut self) {
        if let Some(worker) = self.worker.take() {
            let _ = worker.cmd_tx.send(WorkerCommand::Stop);
            if let Some(join) = worker.join {
                let _ = join.join();
            }
        }
        self.running = false;
    }

    fn drain_events(&mut self) {
        if let Some(worker) = &self.worker {
            while let Ok(evt) = worker.event_rx.try_recv() {
                match evt {
                    WorkerEvent::Info(msg) => self.status = msg,
                    WorkerEvent::Error(msg) => {
                        self.status = msg;
                        self.running = false;
                    }
                }
            }
        }
    }

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

        if self
            .tray_restore_requested
            .swap(false, Ordering::SeqCst)
        {
            self.restore_from_tray(ctx);
        }

    }

    fn minimize_to_tray(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        if let Some(tray_icon) = &self.tray_icon {
            let _ = tray_icon.set_visible(true);
        }
    }

    fn restore_from_tray(&mut self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        if let Some(tray_icon) = &self.tray_icon {
            let _ = tray_icon.set_visible(false);
        }
    }
}

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

fn window_hwnd_from_context(cc: &CreationContext<'_>) -> Option<HWND> {
    let handle = cc.window_handle().ok()?;
    match handle.as_raw() {
        RawWindowHandle::Win32(win32) => Some(HWND(win32.hwnd.get())),
        _ => None,
    }
}

impl Drop for WallpaperApp {
    fn drop(&mut self) {
        self.stop_worker();
    }
}

impl eframe::App for WallpaperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

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

fn default_tray_icon() -> Option<Icon> {
    let size = 16u32;
    let mut rgba = Vec::with_capacity((size * size * 4) as usize);
    // Simple fallback icon so the tray is always visible.
    for y in 0..size {
        for x in 0..size {
            let border = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let (r, g, b) = if border { (255, 255, 255) } else { (60, 120, 220) };
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    }
    Icon::from_rgba(rgba, size, size).ok()
}

fn run_worker(
    images: Vec<PathBuf>,
    auto_rotate: bool,
    style: StyleMode,
    interval: Duration,
    random_order: bool,
    cmd_rx: Receiver<WorkerCommand>,
    evt_tx: Sender<WorkerEvent>,
) -> Result<()> {
    if let Err(err) = set_wallpaper_style(style) {
        let _ = evt_tx.send(WorkerEvent::Error(err.to_string()));
        return Ok(());
    }
    let mut last: Option<PathBuf> = None;
    let mut rng = ChaChaRng::from_entropy();
    loop {
        if matches!(cmd_rx.try_recv(), Ok(WorkerCommand::Stop)) {
            break;
        }

        let next = if random_order {
            pick_random_with_rng(&images, last.as_ref(), &mut rng)?
        } else {
            sequential_pick(&images, last.as_ref())
        };

        let processed = process_image(&next, auto_rotate)?;
        if let Err(err) = set_wallpaper(&processed) {
            let _ = evt_tx.send(WorkerEvent::Error(err.to_string()));
            break;
        }
        let _ = evt_tx.send(WorkerEvent::Info(format!(
            "Set: {}",
            next.display()
        )));
        last = Some(next);

        if wait_or_stop(&cmd_rx, interval) {
            break;
        }
    }
    Ok(())
}

fn wait_or_stop(cmd_rx: &Receiver<WorkerCommand>, interval: Duration) -> bool {
    let step = Duration::from_millis(500);
    let start = Instant::now();
    while start.elapsed() < interval {
        if matches!(cmd_rx.try_recv(), Ok(WorkerCommand::Stop)) {
            return true;
        }
        let remaining = interval.saturating_sub(start.elapsed());
        thread::sleep(step.min(remaining));
    }
    false
}

fn sequential_pick(images: &[PathBuf], last: Option<&PathBuf>) -> PathBuf {
    if images.is_empty() {
        return PathBuf::new();
    }
    if let Some(last) = last {
        if let Some(pos) = images.iter().position(|p| p == last) {
            return images[(pos + 1) % images.len()].clone();
        }
    }
    images[0].clone()
}

fn pick_random_with_rng(
    images: &[PathBuf],
    last: Option<&PathBuf>,
    rng: &mut ChaChaRng,
) -> Result<PathBuf> {
    if images.len() == 1 {
        return Ok(images[0].clone());
    }
    for _ in 0..5 {
        let candidate = images
            .choose(rng)
            .ok_or_else(|| anyhow::anyhow!("no images available"))?;
        if Some(candidate) != last {
            return Ok(candidate.clone());
        }
    }
    images
        .choose(rng)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no images available"))
}

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
