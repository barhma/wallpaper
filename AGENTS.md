# Repository Guidelines

This repo is a Windows-only Rust desktop app for managing wallpapers via an egui GUI and Windows registry integration. Keep changes focused on Windows behavior and UI stability.

## Project Structure & Module Organization

- `src/main.rs`: application entry point, temp file cleanup on startup.
- `src/app/mod.rs`: GUI and slideshow control flow, reset to defaults.
- `src/image_ops/mod.rs`: file discovery, random selection, image transforms, stitching, cropping, temp cleanup.
- `src/wallpaper/wallpaper.rs`: Windows wallpaper styles and setter logic.
- `src/i18n/mod.rs`: English/Traditional Chinese strings.
- `src/settings/mod.rs`: persisted settings model including stitching options.
- `src/startup/mod.rs`: Windows startup registry integration.
- `src/state/mod.rs`: runtime state management.
- `src/slideshow/mod.rs`: background slideshow worker with stitching support.
- `src/theme/mod.rs`: light/dark theme application.
- `Cargo.toml`/`Cargo.lock`: dependencies; build output goes to `target/`.

## Build, Test, and Development Commands

- `cargo run`: build and launch the debug app.
- `cargo build --release`: optimized release build.
- `cargo test`: run Rust tests (none currently in the repo, add as needed).
- `cargo check`: fast compile pass without producing a binary.
- `cargo fmt`: format Rust sources (requires `rustfmt`).
- `cargo clippy`: lint Rust sources (requires `clippy`).

## Coding Style & Naming Conventions

- Use standard Rust formatting (rustfmt defaults, 4-space indent). Run `cargo fmt` after formatting changes.
- Naming: `snake_case` for modules/functions, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.

## Testing Guidelines

- Prefer unit tests inside modules with `#[cfg(test)] mod tests`.
- For integration tests, add files under `tests/` named like `wallpaper_tests.rs`.
- Keep test data small and local to the test module.

## Commit & Pull Request Guidelines

- History uses short, plain messages like "bug fix" or "init"; keep commits concise and descriptive.
- PRs should include a summary, steps to verify, and screenshots for UI changes. Link issues when applicable.

## Platform & Configuration Notes

- Wallpaper styles are applied via `HKCU\Control Panel\Desktop`; avoid touching other registry hives.
- The app writes a BMP cache under the user profile; do not commit generated files or `target/` artifacts.
- Theme selection (default: dark), window opacity, and slideshow state are stored in `settings.json`.
- While running, changes to folders or slideshow options should take effect immediately.
- Temp files are cleaned on startup and managed automatically.

## Features Summary

- **Image Stitching**: Combines 2-5 images with smart rotation patterns
  - Horizontal: 2→[VV], 3→[VHV], 4→[2x2], 5→[3V+2H]
  - Vertical: 2→[HH], 3→[HHH], 4→[2x2], 5→[3H+2V]
- **Auto Scale & Crop**: Always scales and center-crops to target resolution (default 5120x1440)
- **Reset to Defaults**: Restores all settings to factory defaults
- **Temp File Cleanup**: Automatic cleanup on startup and after wallpaper changes
- **Window Opacity**: Persisted and applied on startup with deferred initialization
