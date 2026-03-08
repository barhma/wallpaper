//! Entry point for the Windows wallpaper manager.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod i18n;
mod image_ops;
mod settings;
mod slideshow;
mod startup;
mod state;
mod theme;
mod wallpaper;

/// Configure the native window and start the egui runtime.
fn main() -> anyhow::Result<()> {
    // Clean up temp files from previous runs
    image_ops::cleanup_temp_files();

    let started_from_startup = std::env::args().any(|arg| arg == "--startup");
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([920.0, 620.0])
            .with_min_inner_size([720.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Wallpaper Manager",
        native_options,
        Box::new(move |cc| Box::new(app::WallpaperApp::new(cc, started_from_startup))),
    )?;
    Ok(())
}
