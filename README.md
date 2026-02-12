# Wallpaper Manager

A Windows-only wallpaper manager built in Rust with a native [egui](https://github.com/emilk/egui) GUI. Features tray integration, image stitching, slideshow automation, and persisted settings — all driven through a lightweight desktop interface.

## Features

- **Multi-folder input** — add one folder or batch-select multiple folders, each with an independent "include subfolders" toggle
- **Single-image selection** — pick a single image file to use directly
- **Slideshow** — random or sequential rotation with a configurable interval (5 – 7,200 seconds)
- **Auto-rotate** — automatically rotate portrait images to landscape
- **Set once** — apply a wallpaper immediately without starting the slideshow
- **Next image** — advance the slideshow manually at any time
- **Windows wallpaper styles** — Fill, Fit, Stretch, Tile, Center, Span
- **Run on startup** — register / unregister via the Windows registry
- **Minimize to tray** — keep the slideshow running in the background (including auto-minimize on boot)
- **Light / dark theme** — dark is the default; persisted across sessions
- **Window opacity** — adjustable transparency (0.3 – 1.0), persisted across sessions
- **Bilingual UI** — English and Traditional Chinese, with automatic CJK font detection
- **Reset to defaults** — restore all settings to factory values in one click
- **Temp file cleanup** — automatic on startup and after wallpaper changes

### Image Stitching

Combine 2 – 5 images into a single wallpaper with smart layout and rotation:

| Count | Horizontal layout    | Vertical layout      |
| ----: | -------------------- | -------------------- |
|     2 | V \| V               | H / H                |
|     3 | V \| H \| V          | H / H / H            |
|     4 | 2 × 2 grid           | 2 × 2 grid           |
|     5 | 3 V top + 2 H bottom | 3 H top + 2 V bottom |

- Always scales and center-crops to a configurable output resolution (default **5120 × 1440**)
- Output width range: **640 – 7,680 px** · height range: **480 – 4,320 px**

### Supported Image Formats

`png` · `jpg` / `jpeg` · `bmp` · `gif` · `tif` / `tiff` · `webp`

## Build and Run

> **Requires:** Rust toolchain (edition 2024). Windows only.

```powershell
# Debug build & launch
cargo run

# Optimized release build
cargo build --release
```

### Development Commands

| Command                 | Description                          |
| ----------------------- | ------------------------------------ |
| `cargo run`             | Build and launch the debug app       |
| `cargo build --release` | Optimized release build              |
| `cargo test`            | Run tests (add as needed)            |
| `cargo check`           | Fast compile check, no binary output |
| `cargo fmt`             | Format sources with rustfmt          |
| `cargo clippy`          | Lint sources with clippy             |

## Usage

1. **Add sources** — click _Add Folder_, _Add Folders_ (batch), or _Add Single Image_.
2. **Configure** — toggle auto-rotate, random order, stitching, and set the slideshow interval.
3. **Start / Stop** — click _Start_ to begin the slideshow, _Stop_ to pause it.
4. **Set once** — apply a single wallpaper instantly without a slideshow.
5. **Next** — advance to the next image immediately (works during slideshow or idle).
6. **Remove / Clear** — remove individual entries or clear all selections.
7. **Theme & opacity** — switch between Light / Dark and adjust window transparency.
8. **Minimize to tray** — keep the slideshow running in the background.
9. **Reset to Defaults** — restore all options to their original values.

> While the slideshow is running, changes to folders, stitching, or style take effect immediately (the worker restarts automatically).

## Settings & Persistence

- Settings are saved under the per-user config directory ([`directories`](https://docs.rs/directories) crate `ProjectDirs`) as **`settings.json`**.
- Persisted state includes: folder list, single image path, slideshow options, language, wallpaper style, theme, window opacity, stitching settings, and running state.
- On startup the app restores the last state and resumes the slideshow if it was running.
- Temporary BMP cache files are cleaned up automatically on startup and after each wallpaper change.

## Startup & Registry Integration

- **Run on startup** registers the app in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` with a `--startup` flag.
- The `--startup` flag enables auto-minimize to tray on boot when that option is also enabled.
- Wallpaper styles are applied via `HKCU\Control Panel\Desktop` (`WallpaperStyle` / `TileWallpaper`).

## Configuration Tips

- If no images are found, check file types and folder permissions.
- If startup registration fails, run the app as a normal desktop user (not elevated).
- The app converts the selected image to BMP and caches it under the user profile directory.

## Project Structure

```
src/
├── main.rs              # Entry point, temp file cleanup on startup
├── app/mod.rs           # GUI layout, slideshow control flow, tray integration
├── image_ops/mod.rs     # Image discovery, random/sequential pick, stitching, cropping
├── wallpaper/
│   ├── mod.rs           # Module re-exports
│   └── wallpaper.rs     # Windows wallpaper styles and setter logic
├── i18n/mod.rs          # English / Traditional Chinese string bundles
├── settings/mod.rs      # Persisted settings model (serde JSON)
├── slideshow/mod.rs     # Background slideshow worker thread
├── startup/mod.rs       # Windows startup registry integration
├── state/mod.rs         # Runtime state management
└── theme/mod.rs         # Light / dark theme application
```

## Dependencies

| Crate                                                                         | Purpose                                                  |
| ----------------------------------------------------------------------------- | -------------------------------------------------------- |
| [`eframe`](https://docs.rs/eframe) / [`egui`](https://docs.rs/egui)           | Native GUI framework (wgpu backend)                      |
| [`image`](https://docs.rs/image)                                              | Image loading, rotation, and format conversion           |
| [`rand`](https://docs.rs/rand) / [`rand_chacha`](https://docs.rs/rand_chacha) | Randomized image selection                               |
| [`rfd`](https://docs.rs/rfd)                                                  | Native file / folder picker dialogs                      |
| [`serde`](https://docs.rs/serde) / [`serde_json`](https://docs.rs/serde_json) | Settings serialization                                   |
| [`tray-icon`](https://docs.rs/tray-icon)                                      | System tray integration                                  |
| [`walkdir`](https://docs.rs/walkdir)                                          | Recursive directory traversal                            |
| [`winreg`](https://docs.rs/winreg)                                            | Windows registry access                                  |
| [`windows`](https://docs.rs/windows)                                          | Win32 API bindings (window management, wallpaper setter) |
| [`directories`](https://docs.rs/directories)                                  | Cross-platform user config paths                         |
| [`anyhow`](https://docs.rs/anyhow)                                            | Error handling                                           |
| [`raw-window-handle`](https://docs.rs/raw-window-handle)                      | HWND extraction for layered window opacity               |

## Contributing

- Keep changes Windows-focused; avoid non-Windows assumptions.
- Update `README.md` and `GEMINI.md` when behavior or commands change.
- Prefer small, focused PRs with clear verification steps.

## License

[MIT](LICENSE) © 2025 barhma
