mod app;
mod i18n;
mod image_ops;
mod settings;
mod startup;
mod wallpaper;

fn main() -> anyhow::Result<()> {
    let started_from_startup = std::env::args().any(|arg| arg == "--startup");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wallpaper Manager",
        native_options,
        Box::new(move |cc| Box::new(app::WallpaperApp::new(cc, started_from_startup))),
    )?;
    Ok(())
}
