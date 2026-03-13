# ADR 0001: Use Embedded ZeroClaw Integration in Tauri

Date: 2026-03-13
Status: Accepted
Owner: z-claw desktop team

## Context

`z-claw` is building a desktop GUI for ZeroClaw.
We need a stable integration mode between the Svelte/Tauri desktop app and ZeroClaw runtime capabilities (provider routing, tools, memory, logs, config).

Two modes were considered:

1. Embedded mode:
   - Tauri Rust layer directly calls ZeroClaw Rust code.
2. Daemon mode:
   - GUI talks to a separately running local ZeroClaw process via HTTP/WebSocket.

Current codebase is already Rust-first at desktop backend (`src-tauri`) and includes ZeroClaw as a git submodule under `crates/zeroclaw`.

## Decision

We choose Embedded mode.

Tauri is the only backend boundary exposed to frontend through typed commands.
Frontend must not call ZeroClaw internals directly and must not implement provider logic.

Initial command boundary:

- `status`
- `chat`
- `config_get`
- `config_set`
- `tools_list`
- `logs_tail`

## Why This Decision

1. Lower operational complexity:
   - No external daemon lifecycle, no extra process monitoring for MVP.
2. Better type and error control:
   - Rust-to-Rust integration allows stricter compile-time guarantees.
3. Faster product iteration:
   - One app package, one runtime path, fewer integration breakpoints.
4. Better UX for desktop-first beta:
   - Reduced startup orchestration and fewer runtime states to explain.

## Trade-offs

Pros:
- Simpler deployment and startup.
- Stronger internal API stability via Rust modules.
- Easier local debugging during early development.

Cons:
- Tighter coupling between desktop shell and runtime internals.
- Future remote/multi-client scenarios may need additional abstraction.

## Consequences

1. `src-tauri` will contain a bridge layer that maps commands to ZeroClaw services.
2. Existing direct OpenAI HTTP path in `src-tauri/src/agent.rs` is transitional and will be removed from production path.
3. Frontend calls only Tauri commands with versioned payload contracts.
4. Security-sensitive data (API keys/tokens) must move away from frontend localStorage.

## Guardrails

1. UI data flow rule:
   - frontend -> tauri command -> bridge -> zeroclaw
2. No frontend direct provider calls.
3. No new mock data writebacks in production stores.
4. New backend capability must be added as explicit command contract before UI usage.

## Revisit Criteria

Re-evaluate this ADR if any of the following becomes true:

1. Need multi-frontend access to a single long-running runtime.
2. Need independent runtime scaling/restart separate from GUI lifecycle.
3. Embedded startup or memory profile becomes unacceptable for target devices.
