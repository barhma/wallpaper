# Wallpaper Manager

Windows wallpaper manager built in Rust with a native GUI (egui).

## Features

- Multi-folder input with per-folder "include subfolders"
- Single-image selection
- Random or sequential rotation
- Auto-rotate portrait images
- Slideshow interval control
- Run on Windows startup
- Minimize to tray (including on app start)
- Windows wallpaper style: Fill, Fit, Stretch, Tile, Center, Span
- English and Traditional Chinese UI

## Build and Run

```powershell
cargo run
```

Release build:

```powershell
cargo build --release
```

## Notes

- The app converts the selected image to BMP and writes it to a cache file under your user profile.
- Wallpaper styles are applied via Windows registry keys in `HKCU\Control Panel\Desktop`.

## Project Structure

- `src/main.rs`: app entry point
- `src/app.rs`: GUI and slideshow control
- `src/image_ops.rs`: file discovery, random pick, image processing
- `src/wallpaper.rs`: Windows wallpaper style + setter
- `src/i18n.rs`: English/Traditional Chinese strings
- `src/settings.rs`: persisted app settings
- `src/startup.rs`: Windows startup registry integration
