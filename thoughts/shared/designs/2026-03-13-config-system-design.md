---
date: 2026-03-13
topic: 'Z-Claw Configuration System'
status: validated
---

# Z-Claw 配置系统设计

## Problem Statement

Z-Claw 需要一个安全的配置系统来管理 API key 等敏感信息。当前前端使用 localStorage 存储 API key，存在安全隐患。

**目标**：

1. API key 不再存储在 localStorage
2. 配置与 ZeroClaw CLI 共享（`~/.zeroclaw/config.toml`）
3. 使用 ZeroClaw 内置的加密机制保护敏感数据
4. 前端通过 Tauri command 安全访问配置

## Constraints

### 技术约束

- 必须复用 ZeroClaw 现有配置系统（TOML 格式 + 加密机制）
- 前端与后端通信仅通过 Tauri command
- 配置文件路径：`~/.zeroclaw/config.toml`

### 安全约束

- API key 永不存入 localStorage 或 IndexedDB
- 加密密钥存储在 `~/.zeroclaw/.secret_key`
- 配置传输仅限本地 IPC

### 兼容性约束

- 配置格式与 ZeroClaw CLI 完全兼容
- 支持现有 ZeroClaw 用户的配置迁移

## Approach

**核心设计**：利用 ZeroClaw 现有配置系统，通过 Tauri bridge 层提供安全的配置访问。

**为什么选择这个方案**：

1. **一致性**：GUI 和 CLI 共享配置，用户无需维护两套配置
2. **安全性**：复用 ZeroClaw 成熟的加密存储方案
3. **简洁性**：无需重新发明配置管理逻辑

**替代方案考虑并拒绝**：

- ❌ 使用 tauri-plugin-store：不提供加密功能，且与 ZeroClaw 配置不兼容
- ❌ 自建配置系统：重复造轮子，增加维护成本

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend Layer                        │
│  ┌─────────────────┐    ┌──────────────────────────┐   │
│  │ SettingsPanel   │    │ config store (memory)    │   │
│  │ - 表单 UI       │───▶│ - 非持久化              │   │
│  │ - 验证反馈      │    │ - 从 Tauri 获取         │   │
│  └─────────────────┘    └──────────────────────────┘   │
└─────────────────────────┬───────────────────────────────┘
                          │ Tauri invoke()
┌─────────────────────────▼───────────────────────────────┐
│                   Tauri Bridge Layer                     │
│  ┌────────────────────────────────────────────────────┐ │
│  │ config.rs                                          │ │
│  │ - config_get() → AppConfig                        │ │
│  │ - config_set(AppConfig) → Result<()>              │ │
│  │ - config_validate() → ValidationResult            │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                 ZeroClaw Config System                   │
│  ┌────────────────────────────────────────────────────┐ │
│  │ ~/.zeroclaw/config.toml                           │ │
│  │ - TOML 格式配置                                   │ │
│  │ - [secrets].encrypt = true (加密存储)             │ │
│  │ - ~/.zeroclaw/.secret_key (加密密钥)              │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Components

### 1. Rust Bridge Layer (`src-tauri/src/bridge/config.rs`)

**职责**：

- 读取/写入 ZeroClaw 配置文件
- 处理 API key 的加密/解密
- 提供类型安全的配置接口

**关键函数**：

- `get_config()` - 读取配置，自动解密 API key
- `set_config(config)` - 保存配置，自动加密 API key
- `validate_config(config)` - 验证配置有效性
- `get_config_path()` - 返回配置文件路径

**依赖**：

- `zeroclaw::Config` - ZeroClaw 配置结构
- `zeroclaw::config::schema` - 配置 schema

### 2. Tauri Commands (`src-tauri/src/lib.rs`)

**新增 Commands**：

```rust
#[tauri::command]
fn config_get() -> Result<AppConfig, String> {
    // 读取配置
}

#[tauri::command]
fn config_set(config: AppConfig) -> Result<(), String> {
    // 保存配置
}

#[tauri::command]
fn config_validate(config: AppConfig) -> Result<(), Vec<String>> {
    // 验证配置
}
```

### 3. Frontend Types (`src/lib/types/config.ts`)

**AppConfig 类型定义**：

```typescript
export interface AppConfig {
	apiKey: string; // 解密后的 API key
	apiUrl: string; // 自定义 API endpoint
	provider: string; // Provider 名称
	model: string; // 默认模型
	temperature: number; // 温度参数
	localModelPath?: string; // 本地模型路径（可选）
}
```

### 4. Frontend Store (`src/lib/stores/app.ts`)

**变化**：

- 移除 `persisted` 存储
- 改为普通 `writable` store
- 初始化时从 Tauri 加载配置

## Data Flow

### 配置加载流程

```
1. App 启动
   ↓
2. SettingsPanel onLoad / App 初始化
   ↓
3. invoke('config_get')
   ↓
4. Rust: 读取 ~/.zeroclaw/config.toml
   ↓
5. Rust: 解密 api_key 字段
   ↓
6. 返回 AppConfig 给前端
   ↓
7. 前端更新 config store
```

### 配置保存流程

```
1. 用户点击"保存配置"
   ↓
2. SettingsPanel 收集表单数据
   ↓
3. invoke('config_set', config)
   ↓
4. Rust: 验证配置有效性
   ↓
5. Rust: 加密 api_key 字段
   ↓
6. Rust: 写入 ~/.zeroclaw/config.toml
   ↓
7. 返回成功/失败结果
   ↓
8. 前端显示反馈（toast）
```

## Error Handling

### 加载错误

| 错误类型       | 处理方式                             |
| -------------- | ------------------------------------ |
| 配置文件不存在 | 返回默认配置，提示用户首次设置       |
| TOML 解析错误  | 显示错误信息，引导用户手动修复或重置 |
| 解密失败       | 提示重新输入 API key                 |
| 权限不足       | 显示详细错误，提供解决方案           |

### 保存错误

| 错误类型     | 处理方式               |
| ------------ | ---------------------- |
| 权限不足     | 显示错误，检查文件权限 |
| 磁盘空间不足 | 显示错误，建议清理     |
| 加密失败     | 记录日志，提示重试     |
| 验证失败     | 显示具体字段错误       |

## Testing Strategy

### Rust 层测试

**单元测试**：

- `get_config()` 正常读取
- `set_config()` 正常写入
- 配置加密/解密正确性
- 边界情况（空配置、损坏文件）

**集成测试**：

- 与 ZeroClaw 配置系统兼容性
- 多次读写一致性

### 前端测试

**组件测试**：

- SettingsPanel 加载配置
- 保存成功/失败反馈
- 表单验证

**E2E 测试**：

- 完整配置保存流程
- 应用重启后配置保持

## Security Considerations

### API Key 保护

1. **存储安全**：
   - 加密存储在 `~/.zeroclaw/config.toml`
   - 加密密钥存储在 `~/.zeroclaw/.secret_key`
   - 文件权限 600 (仅用户可读写)

2. **传输安全**：
   - 仅通过 Tauri IPC 传输（本地进程）
   - 不经过任何网络层

3. **前端限制**：
   - 不存储在 localStorage/IndexedDB
   - 不记录到浏览器控制台
   - 不出现在任何日志中

### 配置文件权限

- 确保配置文件权限正确设置
- 加密密钥文件仅用户可访问

## Migration

### 从 localStorage 迁移

如果用户之前使用 localStorage 存储配置：

1. 检测 localStorage 中是否存在旧配置
2. 提示用户迁移
3. 调用 `config_set` 保存到 ZeroClaw 配置
4. 清除 localStorage 中的旧配置

### 迁移脚本

在 Rust bridge 层提供 `migrate_from_localStorage()` 函数，前端检测到旧配置时调用。

## Open Questions

1. **多账户支持时机**？
   - 当前设计支持单账户
   - ZeroClaw 有 auth-profiles 机制
   - 建议：P2 阶段考虑

2. **配置同步 UI**？
   - 是否需要显示"配置已保存"的持久化指示
   - 建议：使用 toast 提示，简单清晰

3. **配置热更新**？
   - ZeroClaw 支持 daemon 模式下热更新配置
   - GUI 是否需要支持？
   - 建议：P1 阶段考虑
