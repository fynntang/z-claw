# ZeroClaw GUI Embedded Mode Sprint Plan

Date: 2026-03-13
Owner: z-claw desktop team
Decision: Confirmed `embedded mode` (Tauri directly calls ZeroClaw Rust code)

## Goal

Build a ZeroClaw GUI frontend where:
- Tauri is the desktop shell and command boundary.
- Core AI/provider/tool/memory logic is handled by ZeroClaw Rust internals.
- Frontend focuses on UX, visibility, and control (not provider implementation).

## Scope and Architecture Boundary

- Integration mode: Embedded.
- Non-goal: Keep a separate OpenAI-direct client path in production logic.
- Tauri command boundary (minimum):
  - `status`
  - `chat`
  - `config_get`
  - `config_set`
  - `tools_list`
  - `logs_tail`

## Priority Roadmap

### P0

1. Finalize embedded architecture and API contract.
2. Build Tauri Rust bridge layer.
3. Replace direct OpenAI HTTP path with ZeroClaw provider/router flow.
4. Implement session + history for multi-turn chat.
5. Make settings page persist and load real config.

### P1

1. Security baseline:
   - move API key out of localStorage
   - tighten Tauri security/capability config
2. Tools page backed by real `tools_list`.
3. Logs page backed by real `logs_tail`.

### P2

1. Remove mock data injection in sidebars and sessions.
2. Add minimal tests and release checklist.

## 7-Day Execution Plan

### Day 1 - Architecture Lock

- Write ADR: `docs/adr/0001-embedded-zeroclaw.md`.
- Define Rust command contract and TS types.
- Freeze naming for the 6 commands.

Acceptance:
- Team agrees on command boundary and no longer changes integration mode.

### Day 2 - Rust Bridge Skeleton

- Create `src-tauri/src/bridge/` (or `core/`) module.
- Separate command handlers from core integration logic.
- Keep current `agent.rs` as temporary fallback only.

Acceptance:
- Command handlers compile and route through a single bridge layer.

### Day 3 - Chat Core Integration

- Replace direct OpenAI `reqwest` flow with ZeroClaw core invocation.
- Add minimal session handling (`session_id`, `history`).
- Ensure model/provider comes from config path.

Acceptance:
- Same session supports contextual follow-up questions.

### Day 4 - Config Center

- Wire `SettingsPanel` to `config_get/config_set`.
- Add config validation and error messaging.
- Ensure restart persistence.

Acceptance:
- Config survives restart and updates runtime behavior.

### Day 5 - Security Baseline

- Remove API key persistence in localStorage.
- Store secrets in OS-level secure storage (or Rust secure layer).
- Tighten `tauri.conf.json` security posture.

Acceptance:
- No plaintext API key in frontend storage.

### Day 6 - Tools and Logs UX

- Implement Tools page from `tools_list`.
- Implement Logs page from `logs_tail`.
- Show errors and execution status in UI.

Acceptance:
- User can inspect tool availability and recent runtime events.

### Day 7 - Cleanup and Beta Gate

- Remove mock data assignment in sidebar/session components.
- Add minimal tests:
  - Rust command tests
  - frontend critical interaction checks
- Run `check/lint/build` and record beta checklist result.

Acceptance:
- Core flow is stable enough for beta testing.

## Risks

1. Tight coupling risk if UI directly depends on ZeroClaw internals.
2. Scope creep from UI polish before backend contract is stable.
3. Security regressions if legacy localStorage secret path remains.

## Mitigations

1. Enforce Tauri command boundary as the only frontend backend interface.
2. Prioritize P0 before any visual refactor.
3. Add a hard check in review: no secret writes in frontend stores.

## Immediate Next Deliverables

1. ADR draft for embedded mode.
2. `src-tauri` bridge module skeleton.
3. Type contract file shared by frontend and Tauri command handlers.
