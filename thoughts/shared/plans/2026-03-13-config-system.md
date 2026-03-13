---
date: 2026-03-13
topic: 'Z-Claw Configuration System Implementation'
status: draft
design_doc: thoughts/shared/designs/2026-03-13-config-system-design.md
---

# 配置系统实现计划

## 概述

本计划将实现安全的配置系统，使 Z-Claw GUI 与 ZeroClaw CLI 共享配置，并使用加密存储保护 API key。

## 任务分解

### Phase 1: Rust Bridge 层 (P0)

#### Task 1.1: 创建配置模块结构

**文件**: `src-tauri/src/bridge/config.rs`

**工作内容**:

- 创建 `config.rs` 模块
- 定义 `AppConfig` 结构体（用于前后端通信）
- 实现与 ZeroClaw `Config` 的转换逻辑

**依赖**: 无

**验证**: `cargo check` 通过

---

#### Task 1.2: 实现配置读取函数

**函数**: `get_config() -> Result<AppConfig, String>`

**工作内容**:

- 获取配置文件路径 (`~/.zeroclaw/config.toml`)
- 读取 ZeroClaw 配置
- 提取 GUI 相关字段
- 处理 API key 解密（如果已加密）
- 处理配置文件不存在的情况（返回默认配置）

**依赖**: Task 1.1

**验证**: 单元测试覆盖各种场景

---

#### Task 1.3: 实现配置保存函数

**函数**: `set_config(config: AppConfig) -> Result<(), String>`

**工作内容**:

- 验证配置有效性
- 加载现有配置（保留非 GUI 字段）
- 更新 GUI 相关字段
- 加密 API key（如果启用加密）
- 写入配置文件
- 确保文件权限正确

**依赖**: Task 1.2

**验证**: 单元测试验证写入和加密

---

#### Task 1.4: 实现配置验证函数

**函数**: `validate_config(config: AppConfig) -> Result<(), Vec<String>>`

**工作内容**:

- 验证 API key 格式（非空）
- 验证 API URL 格式（有效 URL）
- 验证模型名称（非空）
- 验证温度范围 (0.0-2.0)

**依赖**: Task 1.1

**验证**: 单元测试覆盖边界情况

---

#### Task 1.5: 添加 Tauri Commands

**文件**: `src-tauri/src/lib.rs`

**工作内容**:

- 添加 `config_get` command
- 添加 `config_set` command
- 添加 `config_validate` command
- 注册到 `generate_handler!`

**依赖**: Task 1.2, 1.3, 1.4

**验证**: `pnpm tauri dev` 启动成功，command 可调用

---

### Phase 2: 前端改造 (P0)

#### Task 2.1: 定义前端类型

**文件**: `src/lib/types/config.ts` (新建)

**工作内容**:

- 定义 `AppConfig` 接口
- 定义 `ConfigValidationResult` 类型
- 定义默认配置常量

**依赖**: 无

**验证**: TypeScript 编译通过

---

#### Task 2.2: 改造配置 Store

**文件**: `src/lib/stores/app.ts`

**工作内容**:

- 移除 `persisted` 导入和使用
- 改为普通 `writable` store
- 添加 `loadConfig()` 函数（调用 `config_get`）
- 添加 `saveConfig()` 函数（调用 `config_set`）
- 修改 `config` store 初始化逻辑

**依赖**: Task 2.1, Task 1.5

**验证**: 配置加载和保存流程正常

---

#### Task 2.3: 更新 SettingsPanel

**文件**: `src/lib/components/SettingsPanel.svelte`

**工作内容**:

- 添加 `onMount` 加载配置
- 修改 `saveConfig()` 调用新的 Tauri command
- 添加保存成功/失败的 toast 提示
- 添加加载状态指示
- 添加验证错误显示

**依赖**: Task 2.2

**验证**: UI 交互正常，配置可保存和加载

---

#### Task 2.4: 更新 ChatArea

**文件**: `src/lib/components/ChatArea.svelte`

**工作内容**:

- 确保从 store 读取配置正确
- 处理配置未加载的情况

**依赖**: Task 2.2

**验证**: 聊天功能正常

---

### Phase 3: 迁移与兼容 (P1)

#### Task 3.1: 实现迁移检测

**文件**: `src/lib/stores/app.ts`

**工作内容**:

- 检测 localStorage 中是否存在旧配置
- 提供迁移函数
- 迁移成功后清除 localStorage

**依赖**: Task 2.2

**验证**: 迁移流程正常

---

#### Task 3.2: 添加迁移 UI

**文件**: `src/lib/components/SettingsPanel.svelte`

**工作内容**:

- 检测到旧配置时显示迁移提示
- 提供一键迁移按钮

**依赖**: Task 3.1

**验证**: UI 正确显示迁移提示

---

### Phase 4: 测试与文档 (P1)

#### Task 4.1: Rust 单元测试

**文件**: `src-tauri/src/bridge/config.rs`

**工作内容**:

- 测试配置读取
- 测试配置写入
- 测试加密/解密
- 测试边界情况

**依赖**: Phase 1 完成

**验证**: `cargo test` 通过

---

#### Task 4.2: 前端组件测试

**文件**: `src/lib/components/__tests__/SettingsPanel.test.ts` (新建)

**工作内容**:

- 测试配置加载
- 测试配置保存
- 测试验证反馈

**依赖**: Phase 2 完成

**验证**: 测试通过

---

#### Task 4.3: 集成测试

**工作内容**:

- 测试完整配置流程
- 测试应用重启后配置保持
- 测试与 ZeroClaw CLI 配置共享

**依赖**: Phase 1, 2 完成

**验证**: 手动测试通过

---

## 执行顺序

```
Phase 1 (Rust Bridge)
├── Task 1.1 ─┬─► Task 1.2 ───► Task 1.3
│             │
│             └─► Task 1.4
│
└─► Task 1.5 (依赖 1.2, 1.3, 1.4)

Phase 2 (Frontend)
├── Task 2.1 ───► Task 2.2 ───► Task 2.3
│                          └──► Task 2.4

Phase 3 (Migration)
├── Task 3.1 ───► Task 3.2

Phase 4 (Testing)
├── Task 4.1
├── Task 4.2
└── Task 4.3
```

## 验收标准

### Phase 1 完成标准

- [ ] Rust 编译无错误
- [ ] Tauri commands 可正常调用
- [ ] 配置文件正确读写
- [ ] API key 正确加密/解密

### Phase 2 完成标准

- [ ] 前端编译无错误
- [ ] 配置可从 Tauri 加载
- [ ] 配置可保存到 Tauri
- [ ] localStorage 不再存储 API key
- [ ] UI 反馈正常

### Phase 3 完成标准

- [ ] 检测到旧配置时提示迁移
- [ ] 迁移流程正常
- [ ] 迁移后 localStorage 清除

### 最终验收

- [ ] 应用启动正常
- [ ] 配置保存和加载正常
- [ ] 配置在应用重启后保持
- [ ] 与 ZeroClaw CLI 配置共享正常
- [ ] 无安全问题（API key 不泄露）

## 风险与缓解

| 风险                  | 影响 | 缓解措施                                   |
| --------------------- | ---- | ------------------------------------------ |
| ZeroClaw 配置格式变化 | 高   | 使用 zeroclaw crate 提供的类型，而非硬编码 |
| 加密密钥丢失          | 高   | 提示用户重新输入 API key                   |
| 配置文件损坏          | 中   | 提供重置选项，引导用户修复                 |
| 权限问题              | 中   | 检测并提示用户解决                         |

## 时间估算

- Phase 1: 2-3 小时
- Phase 2: 1-2 小时
- Phase 3: 30 分钟
- Phase 4: 1 小时

**总计**: 4-6 小时
