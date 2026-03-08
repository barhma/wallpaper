# Wallpaper Manager

A Windows wallpaper manager built in Rust with a native `egui` desktop UI.

It is designed to be simple to use:

- add folders or a single image
- rotate wallpapers randomly or in order
- apply wallpaper once or run a slideshow
- auto-rotate portrait images
- choose Windows wallpaper style
- save settings and restore them on next launch
- support tray behavior, startup launch, theme, and opacity

## What It Can Do

- folder list with per-folder `include subfolders`
- single-image source
- random or sequential slideshow
- `Next` button for immediate change
- wallpaper styles: `Fill`, `Fit`, `Stretch`, `Tile`, `Center`, `Span`
- two built-in themes:
  - `Dark`: purple-accent dark workspace
  - `Light`: warm beige / rice-color workspace
- adjustable window opacity
- run on startup
- minimize to tray
- English and Traditional Chinese UI
- optional multi-image stitching with crop-based output sizing

## Project Layout

```text
src/
├─ main.rs              # App entry point
├─ app/                 # egui UI and app orchestration
├─ image_ops/           # Image discovery, processing, stitching, temp cleanup
├─ slideshow/           # Background slideshow worker
├─ settings/            # JSON settings model and persistence
├─ startup/             # Windows startup registry integration
├─ state/               # Runtime state mapped from settings
├─ theme/               # egui theme helpers
├─ i18n/                # English / Traditional Chinese strings
└─ wallpaper/           # Windows wallpaper style + apply logic
```

## Build And Run

Windows only.

```powershell
cargo run
```

Release build:

```powershell
cargo build --release
```

Useful development commands:

```powershell
cargo check
cargo fmt
cargo clippy
```

## How It Works

1. Add one or more folders, or choose a single image.
2. Configure slideshow interval, order, wallpaper style, theme, and optional stitching.
3. Click `Set once` to apply immediately or `Start` to run the slideshow.
4. The app stores your settings in the per-user config directory and restores them on next launch.

## UI Layout

- top toolbar: language, theme, window opacity
- action row: `Set once`, `Next`, `Start/Stop`, `Reset to Defaults`
- sources pane: folder/image management and source list
- settings pane: slideshow, wallpaper style, and startup behavior
- bottom status bar: current app state and errors

## Data And Cache Paths

- settings: per-user `settings.json` via the `directories` crate
- generated wallpaper cache / temp files: created under the user profile and cleaned automatically when needed

## Contributor Notes

- This is the active codebase. Old C++ rewrite files were removed.
- Keep changes Windows-focused.
- Do not commit build output like `target/` or temporary files.
- Keep the current compact utility-style layout; avoid dashboard-style redesigns unless explicitly requested.
- If behavior changes, update [README.md](/D:/Codes/wallpaper/README.md) and [AGENTS.md](/D:/Codes/wallpaper/AGENTS.md).

## License

[MIT](LICENSE)
