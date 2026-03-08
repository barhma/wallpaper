# Repository Guidelines

This repo is a Windows-only Rust desktop app for wallpaper management. Keep changes focused on Rust code in `src/`, Windows behavior, and UI stability.

## Project Structure & Module Organization

- `src/main.rs`: app entry point and native window setup.
- `src/app/`: egui UI, tray handling, and app orchestration.
- `src/image_ops/`: image discovery, transforms, stitching, temp cleanup.
- `src/slideshow/`: background slideshow worker.
- `src/settings/`: persisted settings model and JSON read/write.
- `src/startup/`: Windows startup registry integration.
- `src/state/`: runtime state derived from persisted settings.
- `src/theme/`: theme application helpers.
- `src/i18n/`: English and Traditional Chinese strings.
- `src/wallpaper/`: wallpaper style and apply logic.

## Build, Test, and Development Commands

- `cargo run`: build and launch the app locally.
- `cargo build --release`: optimized release build.
- `cargo check`: fast compile validation.
- `cargo fmt`: format code.
- `cargo clippy`: lint code.

## Coding Style & Naming Conventions

- Use standard Rust formatting with 4-space indentation.
- Use `snake_case` for modules and functions, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
- Prefer detailed doc-style comments on public APIs and non-obvious UI or Windows-specific logic.
- Keep modules focused: UI in `app`, persistence in `settings`, OS integration in `startup` and `wallpaper`.

## Testing Guidelines

- There is no full automated test suite yet; validate changes by running the app manually on Windows.
- After touching slideshow or wallpaper code, verify `Set once`, `Next`, `Start`, `Stop`, tray minimize/restore, theme switch, and startup toggle.
- Add unit tests near the changed module when logic can be isolated.

## Commit & Pull Request Guidelines

- Keep commit messages short and descriptive, for example `fix slideshow next handling`.
- PRs should include a summary, verification steps, and screenshots for visible UI changes.

## Platform & Repo Notes

- Target Windows 10/11 only.
- Do not commit generated output such as `target/`, temp files, or local editor artifacts.
- Preserve the compact utility-style UI. Current themes are `Dark` (purple accent) and `Light` (warm beige / rice-tone).
- Prefer practical desktop-tool layouts over dashboard or landing-page styling.
- Update `README.md` and this file when structure, commands, or workflow changes.
