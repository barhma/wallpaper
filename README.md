# Wallpaper Manager

Windows wallpaper manager built in Rust with a native GUI (egui). Designed for Windows-only wallpaper control with tray integration and persisted settings.

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
- Persisted selections and slideshow state between runs
- Light/dark theme with dark as the default
- Next image button for manual advance
- Window opacity control
- Reset to defaults button

### Auto Image Stitching

- Stitch 2-5 images together automatically
- Smart rotation based on layout:
  - Horizontal: 2→[VV], 3→[VHV], 4→[2x2 grid], 5→[3V+2H]
  - Vertical: 2→[HH], 3→[HHH], 4→[2x2 grid], 5→[3H+2V]
- Auto scale and center-crop to target resolution (default: 5120x1440)
- Configurable output width and height

## Build and Run

```powershell
cargo run
```

Release build:

```powershell
cargo build --release
```

## Development Commands

- `cargo run`: build and launch the app.
- `cargo build --release`: optimized release build.
- `cargo test`: run tests (if/when added).
- `cargo fmt`: format Rust sources.
- `cargo clippy`: lint Rust sources.

## Usage

- Add folders or a single image, then click Start to begin the slideshow.
- Click Next to advance the slideshow immediately (or set one image when idle).
- Use Remove next to a folder or single image to delete that entry.
- Use Clear to reset all selections.
- Use the Theme selector (Light/Dark). Dark is the default.
- Minimize to tray to keep the slideshow running in the background.
- While the slideshow is running, changes to folders or options apply immediately.
- Enable "Auto stitch images" to combine multiple images into one wallpaper.
- Click "Reset to Defaults" to restore all settings to their original values.

## Configuration Tips

- If no images are found, check file types (`.jpg`, `.png`, `.bmp`, `.webp`, etc.) and folder permissions.
- If startup registration fails, run once as a normal desktop user (not elevated).

## Settings & Persistence

- Settings are saved under the user config directory (ProjectDirs) as `settings.json`.
- Saved state includes folder list, single image, slideshow options, language, style, stitching settings, and whether the slideshow is running.
- Theme selection and window opacity are saved and restored on launch.
- On startup, the app restores the last state and resumes the slideshow if it was running.
- Temporary cache files are automatically cleaned up on startup and after wallpaper changes.

## Notes

- The app converts the selected image to BMP and writes it to a cache file under your user profile.
- Wallpaper styles are applied via Windows registry keys in `HKCU\Control Panel\Desktop`.
- Temporary files are automatically cleaned to prevent disk space buildup.

## Startup Integration

- When "Run on startup" is enabled, the app registers itself in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- Startup launches include the `--startup` flag to support minimizing to tray on boot.

## Project Structure

- `src/main.rs`: app entry point, temp file cleanup
- `src/app/mod.rs`: GUI and slideshow control
- `src/image_ops/mod.rs`: file discovery, random pick, image processing, stitching, cropping
- `src/wallpaper/wallpaper.rs`: Windows wallpaper style + setter
- `src/i18n/mod.rs`: English/Traditional Chinese strings
- `src/settings/mod.rs`: persisted app settings
- `src/startup/mod.rs`: Windows startup registry integration
- `src/state/mod.rs`: runtime state management
- `src/slideshow/mod.rs`: background slideshow worker
- `src/theme/mod.rs`: light/dark theme application

## Contributing

- Keep changes Windows-focused; avoid non-Windows assumptions.
- Update `README.md` and `AGENTS.md` when behavior or commands change.
- Prefer small, focused PRs with clear verification steps.
