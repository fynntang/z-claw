# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & check

```bash
cargo build                  # default-member: apps/z-claw (GPUI desktop)
cargo build -p z-claw-cli    # CLI binary
cargo build --workspace      # all crates
cargo check --workspace      # fast compile check
cargo clippy --workspace     # clippy is warn-by-default workspace-wide
cargo test --workspace       # run tests (none exist yet)
```

`edition = "2024"` requires Rust 1.85+. `rusqlite` bundles SQLite (needs a C compiler).

## Architecture

Workspace: 2 apps + 12 library crates under `apps/` and `crates/`.

| Layer | Crates |
|-------|--------|
| UI | `z-claw-ui` (GPUI views), `z-claw-assets`, `z-claw-icons`, `z-claw-i18n` |
| Agent | `z-claw-agent` (AgentLoop), `z-claw-tools`, `z-claw-skills`, `z-claw-providers` |
| Core | `z-claw-core` (types, errors, events, Platform trait), `z-claw-config`, `z-claw-security`, `z-claw-memory` |

GPUI is pinned to the Zed repo at tag `v1.4.4` — check Cargo.lock before bumping. Windows may need C++ build tools for native GPUI deps.

### GPUI component patterns (v1.4.4)

- **`IntoElement`** — derived with `#[derive(IntoElement)]` from `gpui_macros`. Use with `RenderOnce` (takes `self`, render gets `&mut App`). Use with `Render` for stateful entities (takes `&mut self`, render gets `&mut Context<Self>`).
- **`on_click`** — signature: `fn(&ClickEvent, &mut Window, &mut App)`. Unlike newer GPUI, the callback does NOT use `cx.listener()` — it's a plain closure.
- **`ui`** crate — Zed's component library (Button, Label, Icon, List, etc.). Use `use ui::prelude::*;` (but avoid clashing `h_flex`/`v_flex` with our own styling).
- **`ui_input`** crate — `InputField::new(window, cx, "placeholder")` for single-line text input. Wraps `Editor::single_line()`.
- Components built with `#[derive(IntoElement)]` + `RenderOnce` can be used directly as children: `div().child(MyComponent { field: value })`
- Theme access in `RenderOnce`: `cx.global::<ThemeColors>()` where `cx: &mut App`

## Conventions

- Conventional commits: `feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:`, `perf:`, `ci:`
- Branch naming: `feat/<name>`, `fix/<name>`, `dev`
- `unsafe_code = "forbid"` workspace-wide — never use `unsafe`
- Default rustfmt (no `rustfmt.toml`) — run `cargo fmt --all` before committing

## Gotchas

- `cargo build` without `--workspace` only builds the desktop app
- UI app hardcodes Ollama at `localhost:11434` with model `llama3` — not wired to config yet
- CLI app loads config from `{config_dir}/z-claw/config.toml`
- Tool approval in agent loop auto-rejects (MVP stub — no UI approval hook yet)
- `.claude/settings.local.json` is gitignored for local overrides

## Remotes

- `origin` — personal fork (`fynntang/z-claw`)
- `upstream` — main repo (`z-claw/z-claw`)

## Theme

Catppuccin Mocha colors hardcoded as hex literals in `z-claw-ui`. Common values:
`#1e1e2e` (bg), `#181825` (sidebar), `#313244` (assistant bubble), `#45475a` (user bubble), `#cdd6f4` (text), `#a6adc8` (muted), `#585b70` (border).
