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
- Use Remove next to a folder or single image to delete that entry.
- Use Clear to reset all selections.
- Minimize to tray to keep the slideshow running in the background.

## Configuration Tips

- If no images are found, check file types (`.jpg`, `.png`, `.bmp`, `.webp`, etc.) and folder permissions.
- If startup registration fails, run once as a normal desktop user (not elevated).

## Settings & Persistence

- Settings are saved under the user config directory (ProjectDirs) as `settings.json`.
- Saved state includes folder list, single image, slideshow options, language, style, and whether the slideshow is running.
- On startup, the app restores the last state and resumes the slideshow if it was running.
- The app also writes a BMP cache file under the user cache directory as `current.bmp`.

## Notes

- The app converts the selected image to BMP and writes it to a cache file under your user profile.
- Wallpaper styles are applied via Windows registry keys in `HKCU\Control Panel\Desktop`.

## Startup Integration

- When "Run on startup" is enabled, the app registers itself in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- Startup launches include the `--startup` flag to support minimizing to tray on boot.

## Project Structure

- `src/main.rs`: app entry point
- `src/app.rs`: GUI and slideshow control
- `src/image_ops.rs`: file discovery, random pick, image processing
- `src/wallpaper.rs`: Windows wallpaper style + setter
- `src/i18n.rs`: English/Traditional Chinese strings
- `src/settings.rs`: persisted app settings
- `src/startup.rs`: Windows startup registry integration

## Contributing

- Keep changes Windows-focused; avoid non-Windows assumptions.
- Update `README.md` and `AGENTS.md` when behavior or commands change.
- Prefer small, focused PRs with clear verification steps.
