# **DO NOT send optional commentary**
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

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
