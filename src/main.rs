mod app;
mod i18n;
mod image_ops;
mod settings;
mod startup;
mod wallpaper;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wallpaper Manager",
        native_options,
        Box::new(|cc| Box::new(app::WallpaperApp::new(cc))),
    )?;
    Ok(())
}
