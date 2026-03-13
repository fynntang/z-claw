# z-claw

Desktop GUI for ZeroClaw, built with `Tauri + SvelteKit`.

## Repo Layout

- `src/`: SvelteKit frontend
- `src-tauri/`: Tauri Rust backend
- `crates/zeroclaw/`: ZeroClaw upstream (git submodule)

## Dependency Strategy

This project uses:

- **Git submodule** for `crates/zeroclaw`
- **Path dependency** from workspace/Tauri to local ZeroClaw code

Why:

1. Local cross-repo debugging is faster.
2. GUI and ZeroClaw can be patched together.
3. Version is pinned by submodule commit.

## Clone and Setup

Clone with submodule:

```bash
git clone --recurse-submodules <repo-url>
```

If already cloned:

```bash
git submodule update --init --recursive
```

Install frontend deps:

```bash
pnpm install
```

## Daily Commands

Update submodule to recorded commit:

```bash
git submodule update --init --recursive
```

Pull latest upstream into submodule (when you intentionally upgrade):

```bash
git submodule update --remote --merge crates/zeroclaw
```

Run app:

```bash
pnpm tauri dev
```

## Verify Workspace

Use one of:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

```bash
pnpm tauri dev
```

## Submodule Commit Workflow

When `crates/zeroclaw` changes:

1. Commit inside submodule first (if needed).
2. Return to root repo.
3. Commit the updated submodule pointer in root repo.

Quick check:

```bash
git submodule status
```

## Notes

- Do not replace submodule with ad-hoc copied code.
- Pin by commit (default submodule behavior), not floating branch references.

## License

This project is dual-licensed under either:

- Apache License, Version 2.0, see `LICENSE-APACHE`
- MIT License, see `LICENSE-MIT`

You may choose the license that best fits your needs.
