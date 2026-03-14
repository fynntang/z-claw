# z-claw

Desktop GUI for ZeroClaw, built with `Tauri + SvelteKit`.

## Repo Layout

- `src/`: SvelteKit frontend
- `src-tauri/`: Tauri Rust backend
- `src-tauri/binaries/`: zeroclaw 嵌入式二进制（构建时由脚本下载或本地编译生成）
- `crates/zeroclaw/`: 可选 git submodule，仅当 `ZEROCLAW_BUILD_FROM_SOURCE=1` 时用于本地编译 zeroclaw

## Dependency Strategy

本项目**不再**通过 Cargo 依赖 zeroclaw 库，改为：

- **嵌入 zeroclaw 二进制**：作为 Tauri sidecar，在应用启动时执行 `zeroclaw gateway`。
- **Gateway HTTP API**：配置、状态、工具列表等均通过 zeroclaw 提供的 REST API（如 `GET/PUT /api/config`、`GET /api/status`、`GET /api/tools`）访问，并配合配对 token（`POST /pair`）认证。

构建时默认从 [zeroclaw Releases](https://github.com/zeroclaw-labs/zeroclaw/releases) 下载当前平台的二进制；若需从源码编译，可设置 `ZEROCLAW_BUILD_FROM_SOURCE=1` 并确保已执行 `git submodule update --init crates/zeroclaw`。

## Clone and Setup

```bash
git clone <repo-url>
cd z-claw
pnpm install
```

若需本地从源码编译 zeroclaw（可选）：

```bash
git submodule update --init crates/zeroclaw
```

## Daily Commands

构建前会自动执行 `pnpm run ensure-zeroclaw` 以准备 sidecar 二进制；也可手动执行：

```bash
pnpm run ensure-zeroclaw
```

运行应用：

```bash
pnpm tauri dev
```

打包：

```bash
pnpm tauri build
```

## Verify

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

或：

```bash
pnpm tauri dev
```

## 使用外部 Gateway（可选）

若本机已单独运行 zeroclaw gateway，可不启动内嵌进程，仅作客户端：

```bash
ZEROCLAW_USE_EXTERNAL_GATEWAY=1 pnpm tauri dev
```

需确保已配对（如曾通过桌面端完成过一次配对，token 保存在 `~/.config/zeroclaw/desktop-token`）。
