//! Entry point for the Windows wallpaper manager.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod i18n;
mod image_ops;
mod slideshow;
mod settings;
mod state;
mod startup;
mod theme;
mod wallpaper;

/// Configure the native window and start the egui runtime.
fn main() -> anyhow::Result<()> {
    let started_from_startup = std::env::args().any(|arg| arg == "--startup");
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([680.0, 600.0])
            .with_min_inner_size([540.0, 460.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Wallpaper Manager",
        native_options,
        Box::new(move |cc| Box::new(app::WallpaperApp::new(cc, started_from_startup))),
    )?;
    Ok(())
}
