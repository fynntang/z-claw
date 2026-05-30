---
name: build-all
description: Build both desktop and CLI targets. Use when you need to verify the full workspace compiles.
---

Run `cargo build -p z-claw -p z-claw-cli` to build both the GPUI desktop app and the CLI binary.

If that fails, fall back to `cargo build --workspace` for a full workspace build.
