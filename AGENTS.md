# Repository Guidelines

This repo is a Windows-only Rust desktop app for managing wallpapers via an egui GUI and Windows registry integration. Keep changes focused on Windows behavior and UI stability.

## Project Structure & Module Organization

- `src/main.rs`: application entry point and runtime wiring.
- `src/app.rs`: GUI and slideshow control flow.
- `src/image_ops.rs`: file discovery, random selection, image transforms.
- `src/wallpaper.rs`: Windows wallpaper styles and setter logic.
- `src/i18n.rs`: English/Traditional Chinese strings.
- `src/settings.rs`: persisted settings model.
- `src/startup.rs`: Windows startup registry integration.
- `src/image/`: image assets used by the app.
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
