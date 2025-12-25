#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod i18n;
mod image_ops;
mod settings;
mod startup;
mod wallpaper;

fn main() -> anyhow::Result<()> {
    let started_from_startup = std::env::args().any(|arg| arg == "--startup");
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([760.0, 720.0])
            .with_min_inner_size([620.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Wallpaper Manager",
        native_options,
        Box::new(move |cc| Box::new(app::WallpaperApp::new(cc, started_from_startup))),
    )?;
    Ok(())
}
