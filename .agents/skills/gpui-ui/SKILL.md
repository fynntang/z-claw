---
name: gpui-ui
description: Design, implement, review, or debug GPUI desktop interfaces in z-claw. Use when changing apps/z-claw or crates/z-claw-ui, building GPUI views/components, fixing focus/click/text input behavior, unifying z-claw visual style, or checking GPUI v1.4.4 patterns.
---

# GPUI UI

## Purpose

Build z-claw desktop UI changes with the repository's GPUI v1.4.4 patterns, visual system, and verification habits. Keep interfaces quiet, dense, and work-focused: z-claw should feel like an agent console or editor surface, not a landing page.

## First Pass

1. Read the local UI shape before editing:
   - `apps/z-claw/src/main.rs` for window composition, focus, and custom input handling.
   - `crates/z-claw-ui/src/theme.rs` for semantic colors.
   - `crates/z-claw-ui/src/components/` for reusable primitives.
   - `crates/z-claw-ui/src/views/` for full view sections.
2. Preserve existing ownership boundaries. Put reusable visual pieces in `z-claw-ui`; keep app state wiring in `apps/z-claw`.
3. Prefer restrained GPUI code over new abstractions unless repetition is already meaningful.
4. Use `apply_patch` for manual edits.

## Design Direction

- Use a consistent console/editor aesthetic: dark neutral backgrounds, sharp hierarchy, compact controls, and clear status colors.
- Keep border radius at 8px or less unless matching an existing component.
- Avoid marketing-style heroes, decorative cards, gradients, blobs, or copy explaining how the app works.
- Avoid nested cards. Use full-width bands, rails, panels, and repeated item cards only where the UI truly represents an item.
- Keep text compact in tool surfaces. Use large type only for app-level empty states or titles.
- Make clickable elements visibly interactive with stable size, hover, and disabled states.

## GPUI v1.4.4 Rules

- `#[derive(IntoElement)]` with `RenderOnce` is good for stateless components that consume `self`.
- Use `Render` for stateful entities; its render receives `&mut Context<Self>`.
- `on_click` callbacks use `Fn(&ClickEvent, &mut Window, &mut App)`. Do not use newer GPUI `cx.listener()` assumptions here.
- `on_mouse_down(MouseButton::Left, ...)` is often useful for simple focus/selection, but prefer component `on_click` when a component already exposes it.
- `track_focus(&focus_handle)` must be on a focusable element that should own keyboard focus.
- Do not call `focus()` unconditionally on every render. Focus once on startup, after submit, or after explicit user actions.
- For custom text input, implementing `EntityInputHandler` is not enough. During the input element's `paint()`, call:

```rust
window.handle_input(
    &focus_handle,
    ElementInputHandler::new(bounds, entity),
    cx,
);
```

- Text selection and IME ranges are UTF-16 based. Convert UTF-16 ranges to byte ranges before replacing Rust `String` content.
- For IME placement, return useful bounds from `bounds_for_range`.

## z-claw Component Conventions

- Theme colors are semantic. Prefer `ThemeColors` fields over ad hoc hex literals.
- Main app composition belongs in `apps/z-claw/src/main.rs`; shared styling belongs in `crates/z-claw-ui`.
- Sidebar, chat, settings, and approval dialogs should share the same surface, border, text, and status language.
- Buttons should use `Button`, `ButtonVariant`, and disabled states instead of handwritten command pills.
- Avoid direct slicing of ids or strings for display. Use safe `chars().take(n)` or other UTF-8 safe handling.
- Do not reintroduce `editor::Editor` unless the task specifically requires rich editor behavior and the Zed settings/theme globals are initialized.

## Verification

Run focused checks after UI work:

```bash
cargo fmt --package z-claw --package z-claw-ui
cargo check -p z-claw
cargo build -p z-claw
```

If `cargo build` fails on Windows with access denied while removing `target\debug\z-claw.exe`, close any running `z-claw` process and rerun the build.

For visual verification, launch `target\debug\z-claw.exe` and confirm startup. Use Computer Use when available for screenshot/click/type checks; if the Windows helper fails, report the exact helper error and the verification that was still completed.

## Common Failure Modes

- Text box renders but cannot type: missing `window.handle_input(...)` in custom input `paint()`.
- Buttons appear unclickable: focus is being stolen every render, propagation is stopped too high in the tree, or a transparent overlay is still mounted.
- IME or Chinese input corrupts text: treating UTF-16 selection ranges as byte ranges.
- UI looks inconsistent: mixing raw colors or old component styles instead of updating theme/primitives and all related views together.
