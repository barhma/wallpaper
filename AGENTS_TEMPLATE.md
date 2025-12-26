# Repository Guidelines Template

codex resume 019b55e1-a2fd-7291-a03f-0a65f06b18e3
Use this file as a starting point when creating a new `AGENTS.md` for a repo.
Keep the final document concise (200-400 words) and specific to the project.

## Engineering Principles

- Prefer professional, layered architecture: separate UI, domain logic, IO, and infrastructure.
- Add detailed doc-style comments for public APIs, modules, and non-obvious logic.
- Use the newest stable versions of core libraries, and skim their official docs for idiomatic usage.
- Follow top-tier code style: clear naming, small functions, single-responsibility modules.

## Project Structure & Module Organization

- Describe the top-level layout (e.g., `src/`, `tests/`, `assets/`).
- Call out key entry points and important modules.

Example:

- `src/main.rs`: app entry point
- `src/app.rs`: UI logic
- `assets/`: static files

## Build, Test, and Development Commands

- List the most common commands and what they do.

Example:

- `cargo run`: build and run locally
- `cargo test`: run tests
- `cargo build --release`: release build

## Coding Style & Naming Conventions

- Mention indentation, formatting tools, and naming rules.
- State comment standards (doc comments for modules/types/functions).

Example:

- 4-space indentation; `snake_case` for functions, `CamelCase` for types.
- Run `cargo fmt` and `cargo clippy` before PRs.
- Prefer `///` doc comments on public APIs and `//!` module docs.

## Testing Guidelines

- State test frameworks, where tests live, and naming patterns.

Example:

- Unit tests in `src/` with `#[cfg(test)]`.
- Integration tests in `tests/`, e.g., `wallpaper_tests.rs`.

## Commit & Pull Request Guidelines

- Summarize commit message conventions.
- List PR requirements (summary, tests, screenshots, linked issues).

Example:

- Short, descriptive commit messages.
- PRs include a summary and verification steps.

## Optional Sections

Add only if relevant:

- Configuration & Environment
- Security Notes
- Architecture Overview
- Agent-Specific Instructions
