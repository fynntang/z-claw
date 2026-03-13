# Submodule and Dependency Guide

Date: 2026-03-13

## Decision

For `z-claw` development, we use:

1. `crates/zeroclaw` as a git submodule.
2. Rust `path` dependencies to integrate ZeroClaw into Tauri.

This is preferred during active product development because desktop and runtime can be changed together and tested immediately.

## When to Prefer This Over Cargo Git Dependency

Choose submodule + path when:

1. You frequently patch both GUI and ZeroClaw.
2. You need deterministic local behavior with minimal network dependence.
3. You want clear pinning via submodule commit pointer.

Choose Cargo git dependency when:

1. You only consume upstream releases.
2. You do not need local runtime modifications.
3. You prefer less repository management overhead.

## Required Local Commands

Initialize:

```bash
git submodule update --init --recursive
```

Check submodule commit:

```bash
git submodule status
```

Move to latest upstream (intentional upgrade only):

```bash
git submodule update --remote --merge crates/zeroclaw
```

## Team Rules

1. Never leave submodule detached at an arbitrary local-only commit without recording intent.
2. Every submodule update must be in its own root-repo commit with upgrade notes.
3. Always run compile check after submodule bump:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

## Common Pitfalls

1. Forgetting `--recurse-submodules` on clone.
2. Updating submodule files but not committing root pointer change.
3. Mixing workspace member rules and nested workspace rules incorrectly.

## Recovery Checklist

If submodule state is inconsistent:

1. `git submodule sync --recursive`
2. `git submodule update --init --recursive`
3. Re-run build/check command.
