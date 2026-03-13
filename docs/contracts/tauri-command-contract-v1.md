# Tauri Command Contract v1 (Embedded ZeroClaw)

Date: 2026-03-13
Version: v1
Status: Draft (for implementation)

## Conventions

1. All commands return:

```ts
type CommandResult<T> = {
  ok: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: string;
  };
};
```

2. Time is ISO-8601 UTC string.
3. All IDs are opaque strings.
4. `error.code` is stable; `error.message` is user-facing.

## 1) status

Purpose: runtime health snapshot for top-bar and diagnostics.

Request:

```ts
type StatusRequest = {};
```

Response data:

```ts
type StatusData = {
  app_version: string;
  runtime_ready: boolean;
  provider: string;
  model: string;
  active_session_id?: string;
  uptime_sec: number;
};
```

## 2) chat

Purpose: send one turn to ZeroClaw runtime with optional session continuity.

Request:

```ts
type ChatRequest = {
  session_id?: string;
  message: string;
  model?: string;
  provider?: string;
};
```

Response data:

```ts
type ChatData = {
  session_id: string;
  reply: string;
  model: string;
  provider: string;
  usage?: {
    prompt_tokens?: number;
    completion_tokens?: number;
    total_tokens?: number;
  };
  tool_calls?: Array<{
    name: string;
    status: "ok" | "error";
    summary?: string;
  }>;
  created_at: string;
};
```

## 3) config_get

Purpose: read effective app/runtime config needed by GUI.

Request:

```ts
type ConfigGetRequest = {};
```

Response data:

```ts
type ConfigView = {
  api_url?: string;
  provider?: string;
  model?: string;
  has_api_key: boolean;
  workspace_path?: string;
};
```

## 4) config_set

Purpose: update config fields from settings panel.

Request:

```ts
type ConfigSetRequest = {
  api_url?: string;
  provider?: string;
  model?: string;
  api_key?: string; // write-only
  workspace_path?: string;
};
```

Response data:

```ts
type ConfigSetData = {
  updated: Array<"api_url" | "provider" | "model" | "api_key" | "workspace_path">;
  requires_restart: boolean;
};
```

## 5) tools_list

Purpose: show available tool surface in Tools page.

Request:

```ts
type ToolsListRequest = {};
```

Response data:

```ts
type ToolItem = {
  name: string;
  description: string;
  enabled: boolean;
  risk: "low" | "medium" | "high";
};
```

## 6) logs_tail

Purpose: show recent runtime events/errors in Logs page.

Request:

```ts
type LogsTailRequest = {
  limit?: number; // default 100, max 1000
  level?: "trace" | "debug" | "info" | "warn" | "error";
};
```

Response data:

```ts
type LogEntry = {
  ts: string;
  level: "trace" | "debug" | "info" | "warn" | "error";
  target: string;
  message: string;
};
```

## Error Codes (initial set)

- `INVALID_INPUT`
- `NOT_CONFIGURED`
- `RUNTIME_UNAVAILABLE`
- `PROVIDER_ERROR`
- `TOOL_ERROR`
- `INTERNAL_ERROR`

## Implementation Notes

1. Rust side should define the canonical DTOs and derive `Serialize`/`Deserialize`.
2. Frontend types should mirror this file, generated or manually synced.
3. v1 is intentionally small; do not add streaming until base path is stable.
