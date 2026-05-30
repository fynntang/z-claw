# z-claw Desktop AI Agent — 实施规划（合并版）

## 七项目分析摘要

| 项目               | 语言         | 定位            | 核心借鉴                                                         |
|------------------|------------|---------------|--------------------------------------------------------------|
| **Pi**           | TypeScript | 编码 Agent CLI  | Harness 模式（Agent Loop 与 UI 解耦）、会话压缩、统一 LLM 抽象                |
| **Hermes Agent** | Python     | 自进化个人助手       | 技能系统（agentskills.io）、多平台消息网关、子 Agent、学习闭环                    |
| **ZeroClaw**     | Rust       | 高性能 Agent 运行时 | 模块化 crate 设计（~20个）、安全分级自治、加密审计、SOP 引擎                        |
| **OpenClaw**     | TypeScript | 个人 AI 助手框架    | 插件 SDK、Memory Host SDK、原生移动端                                 |
| **Claude Code**  | TypeScript | 生产级编码 Agent   | Skills/Hooks/Memory 系统、五级配置模型、Plan mode、Task 抽象层、前景/后台 Agent |
| **Claw Code**    | Rust       | Claude Code Rust 重写 | **最接近的参考**: PermissionPolicy、Session(JSONL)、MCP stdio、三产品架构 |
| **Zed**          | Rust       | GPUI 原生编辑器    | Entity-Context 模式、Panel/Dock 系统、主题系统、组件库、异步模式、Keybinding     |

### 关键学习点

1. **Harness 模式**（Pi）：Agent Loop 与 UI 完全解耦，通过事件流通信 —— 最干净的设计
2. **模块化 Crate 拆分**（ZeroClaw + Claw Code）：细粒度 crate 而非单体文件（对比 Hermes 697KB 的 cli.py）
3. **多供应商抽象层**：所有项目的共同需求
4. **安全分级自治**（ZeroClaw + Claw Code）：PermissionMode/PermissionPolicy，桌面应用的硬需求
5. **技能系统**（Hermes + Claude Code）：agentskills.io + SKILL.md frontmatter 格式，条件路径激活
6. **Hooks 系统**（Claude Code）：26 种生命周期事件、4 种 hook 类型、matcher 条件过滤
7. **GPUI 架构**（Zed）：Entity-Context 模式、Panel/Dock 停靠、PaneGroup 布局树、主题色语义系统
8. **记忆系统**（Claude Code）：四类型 taxonomy、MEMORY.md 索引 + 话题文件双层结构
9. **三产品架构**（Claw Code）：完整 CLI + 轻量 analog + 独立 RAG 服务，清晰边界分离

---

## 1. 架构总览

```
┌──────────────────────────────────────────────────────────────┐
│                      GPUI Desktop UI                          │
│  ┌──────────┐ ┌──────────┐ ┌────────────┐ ┌──────────────┐ │
│  │ Chat View│ │Agent Mgmt│ │   Skills   │ │Code Diff     │ │
│  └──────────┘ └──────────┘ └────────────┘ └──────────────┘ │
├──────────────────────────────────────────────────────────────┤
│                     z-claw-ui (App Shell)                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────────┐ ┌────────────┐ │
│  │Workspace │ │ Settings │ │ Keybindings  │ │ Theme      │ │
│  └──────────┘ └──────────┘ └──────────────┘ └────────────┘ │
├──────────────────────────────────────────────────────────────┤
│                   z-claw-agent (Agent Runtime)                │
│  ┌──────────┐ ┌──────────┐ ┌──────────────┐ ┌────────────┐ │
│  │Agent Loop│ │ Harness  │ │Plan Mode     │ │  Hooks     │ │
│  │          │ │          │ │(Enter/Exit/  │ │(Pre/Post/  │ │
│  │          │ │          │ │ Verify)      │ │ Lifecycle) │ │
│  └──────────┘ └──────────┘ └──────────────┘ └────────────┘ │
├──────────┬──────────┬──────────┬──────────┬─────────────────┤
│Providers │  Tools   │  Skills  │  Memory  │   Security      │
│(10+ LLM) │(shell,fs │(SKILL.md │(4-type   │(0-4 levels,    │
│          │ ,http,..)│ +paths)  │ taxonomy)│ sandbox,audit)  │
├──────────┴──────────┴──────────┴──────────┴─────────────────┤
│                 Platform Abstraction                          │
│            (fs, shell, network, process)                      │
└──────────────────────────────────────────────────────────────┘
```

---

## 2. Crate 拆分

```
z-claw/
  Cargo.toml
  crates/
    z-claw-core/            # 领域类型、trait、错误（零运行时依赖）
    z-claw-agent/           # Agent 运行时：循环、Harness、会话、子Agent、审批
    z-claw-providers/       # LLM 供应商抽象层 + 多供应商实现
    z-claw-tools/           # Tool trait + 内置工具 + MCP 客户端
    z-claw-skills/          # 技能系统（agentskills.io 兼容）
    z-claw-memory/          # 记忆：SQLite + FTS5 + embeddings + RAG
    z-claw-security/        # 安全策略、沙箱、审计日志
    z-claw-config/          # TOML 配置加载 + 实时重载
    z-claw-ui/              # GPUI 视图和组件
  apps/
    z-claw/                 # 桌面二进制入口
```

### Crate 职责矩阵

| Crate              | 职责                                                                                         | 关键依赖                            |
|--------------------|--------------------------------------------------------------------------------------------|---------------------------------|
| `z-claw-core`      | `LlmProvider` trait, `Tool` trait, `Skill` trait, `MemoryBackend` trait, 共享类型, `ClawError` | serde, async-trait, futures     |
| `z-claw-agent`     | Agent Loop（双循环）、Harness（组合 providers+tools+skills）、会话管理、上下文压缩、子Agent 管理                    | z-claw-core, tokio              |
| `z-claw-providers` | Anthropic, OpenAI, Google, DeepSeek, Ollama 适配器、供应商路由/故障转移                                 | z-claw-core, reqwest            |
| `z-claw-tools`     | Shell, 文件系统, HTTP, 浏览器, MCP 协议客户端、ToolRegistry                                             | z-claw-core, rmcp               |
| `z-claw-skills`    | Skill trait, 注册表, YAML/Markdown 加载器, 技能创建向导, agentskills.io 兼容                             | z-claw-core, serde_yaml         |
| `z-claw-memory`    | SQLite + FTS5 全文搜索, 向量嵌入, RAG 检索, 会话摘要压缩                                                   | z-claw-core, rusqlite           |
| `z-claw-security`  | 5 级安全分级, 沙箱执行, 操作审计日志                                                                      | z-claw-core                     |
| `z-claw-config`    | TOML 配置加载, schema 定义, 文件热重载                                                                | z-claw-core, serde, notify      |
| `z-claw-ui`        | GPUI AppModel, 聊天/Agent/技能/设置/差异视图, Markdown 渲染                                            | z-claw-core, z-claw-agent, gpui |
| `apps/z-claw`      | GPUI App 启动, 窗口管理, 内核初始化                                                                   | all crates, gpui, tracing       |

---

## 3. 核心类型设计

### Provider 抽象

```rust
// z-claw-providers

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Stream a chat completion. Returns text deltas + tool call fragments.
    async fn chat(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolDef>,
        config: &GenerateConfig,
    ) -> Result<StreamedResponse>;

    fn list_models(&self) -> Vec<ModelInfo>;
    fn supports_feature(&self, feature: Feature) -> bool; // vision, tool_calling, reasoning
}

pub struct StreamedResponse {
    pub stream: Pin<Box<dyn Stream<Item=Result<StreamChunk>> + Send>>,
}

pub enum StreamChunk {
    TextDelta(String),
    ToolCallStart { index: usize, id: String, name: String },
    ToolCallDelta { index: usize, args_delta: String },
    ToolCallEnd { index: usize },
    ThinkingDelta(String),
    Usage(UsageInfo),
    Done,
}
```

### Agent Loop（Harness 模式，UI 无关）

```rust
// z-claw-agent

pub struct AgentLoop {
    harness: Arc<Harness>,
    session: Session,
    approval: ApprovalGate,
    policy: PolicyConfig,
}

pub struct Harness {
    provider_chain: ProviderChain,       // 主 + 备用供应商
    tools: Arc<ToolRegistry>,
    skills: Arc<SkillRegistry>,
    memory: Arc<dyn MemoryBackend>,
    system_prompt: String,
}

impl AgentLoop {
    /// Run one user turn through the agent loop.
    /// Returns events via the provided sender; UI thread consumes them.
    pub async fn run_turn(
        &mut self,
        user_input: &str,
        event_tx: &mpsc::UnboundedSender<AgentEvent>,
    ) -> Result<AgentResponse> {
        // 1. Persist user message
        self.session.add_message("user", user_input);

        // 2. Inner loop: model ↔ tool calls
        for _round in 0..MAX_ROUNDS {
            // Build context (system prompt + history + tools + injected memories)
            let context = self.build_context().await?;

            // Stream LLM response
            let mut response = self.harness.provider_chain.chat(context).await?;

            let mut text = String::new();
            let mut tool_calls = ToolCallAccumulator::new();

            while let Some(chunk) = response.stream.next().await {
                match chunk? {
                    StreamChunk::TextDelta(delta) => {
                        text.push_str(&delta);
                        let _ = event_tx.send(AgentEvent::TextDelta(delta));
                    }
                    StreamChunk::ToolCallStart { index, id, name } => {
                        tool_calls.start(index, id, name);
                        let _ = event_tx.send(AgentEvent::ToolCallStarted { name: name.clone() });
                    }
                    StreamChunk::ToolCallDelta { index, args_delta } => {
                        tool_calls.push_args(index, &args_delta);
                    }
                    StreamChunk::ToolCallEnd { index } => {
                        tool_calls.finish(index);
                    }
                    StreamChunk::Done => break,
                    _ => {}
                }
            }

            // If no tool calls, we're done
            let calls = tool_calls.finalize();
            if calls.is_empty() {
                self.session.add_message("assistant", &text);
                return Ok(AgentResponse::Text(text));
            }

            // Execute tool calls with security approval
            for call in calls {
                // Check security level
                let sec_level = self.policy.classify(&call);
                if sec_level >= SecurityLevel::RequireApproval {
                    let _ = event_tx.send(AgentEvent::ApprovalRequired {
                        tool_name: call.name.clone(),
                        arguments: call.arguments.clone(),
                    });
                    // Wait for user approval (or timeout)
                    // ...
                }

                let result = self.harness.tools.execute(&call).await?;
                self.session.add_tool_result(&call, &result);
                let _ = event_tx.send(AgentEvent::ToolCallFinished {
                    name: call.name,
                    ok: result.is_ok(),
                });
            }
            // Continue loop — feed tool results back to model
        }

        Ok(AgentResponse::Text(text))
    }
}
```

---

## 4. 安全分级自治（借鉴 ZeroClaw）

```rust
// z-claw-security

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Level 0: 自动执行 — 读取文件、搜索代码、查看状态
    AutoExecute = 0,
    /// Level 1: 确认执行 — 修改文件、安装依赖、创建目录
    ConfirmExecute = 1,
    /// Level 2: 沙箱执行 — 运行任意 shell 命令（在容器/沙箱中）
    SandboxExecute = 2,
    /// Level 3: 需要审批 — 网络请求外部服务、git push、删除文件
    RequireApproval = 3,
    /// Level 4: 禁止 — rm -rf /、修改系统配置、访问敏感路径
    Blocked = 4,
}

pub trait SecurityPolicy: Send + Sync {
    fn classify(&self, tool_name: &str, args: &Value, path_context: &Path) -> SecurityLevel;
    fn is_path_allowed(&self, path: &Path) -> bool;
    fn is_command_allowed(&self, command: &str) -> bool;
}

// 默认策略
impl DefaultSecurityPolicy {
    pub fn new(config: PolicyConfig) -> Self { ... }

    fn classify_impl(&self, tool_name: &str, args: &Value) -> SecurityLevel {
        match tool_name {
            // Level 0: Read-only operations
            "read_file" | "list_directory" | "search_code" | "read_memory" => SecurityLevel::AutoExecute,

            // Level 1: File modifications
            "write_file" | "create_directory" => SecurityLevel::ConfirmExecute,

            // Level 2: Shell commands (sandboxed)
            "execute_command" if self.is_safe_command(args) => SecurityLevel::ConfirmExecute,
            "execute_command" => SecurityLevel::SandboxExecute,

            // Level 3: External interactions
            "http_request" | "git_push" | "browser_navigate" => SecurityLevel::RequireApproval,

            // Level 4: System-level changes
            "system_config" | "install_package" => SecurityLevel::Blocked,

            _ => SecurityLevel::ConfirmExecute,
        }
    }
}
```

### 审批流

```
Tool Call → classify() → SecurityLevel
  │
  ├─ AutoExecute     → 立即执行，无中断
  ├─ ConfirmExecute  → UI 内联提示（可设置"本次会话自动允许"）
  ├─ SandboxExecute  → 在沙箱中执行，UI 显示沙箱状态
  ├─ RequireApproval → UI 弹出审批对话框，120s 超时自动拒绝
  └─ Blocked         → 直接拒绝，显示原因
```

### Claw Code 权限系统参考（Rust 实现）

Claw Code 的 `PermissionPolicy` 是 z-claw 安全系统最直接的参考：

```rust
// Claw Code: rust/crates/runtime/src/permissions.rs:99
pub struct PermissionPolicy {
    active_mode: PermissionMode,
    tool_requirements: BTreeMap<String, PermissionMode>,  // per-tool 最低权限
    allow_rules: Vec<PermissionRule>,
    deny_rules: Vec<PermissionRule>,
    ask_rules: Vec<PermissionRule>,
    denied_tools: Vec<String>,
}
```

**Session JSONL 持久化**（`rust/crates/runtime/src/session.rs:117`）：
```rust
pub struct Session {
    pub session_id: String,
    pub messages: Vec<ConversationMessage>,
    pub compaction: Option<SessionCompaction>,
    pub fork: Option<SessionFork>,
    pub workspace_root: Option<PathBuf>,
    pub model: Option<String>,
    persistence: Option<SessionPersistence>,  // JSONL 文件路径
}
```

Session 使用原子写入 + 日志轮转的 `save_to_path()` 持久化策略。

### Crate 映射总表

| Claw Code Crate | z-claw Crate | 设计参考 |
|-----------------|-------------|----------|
| `api` | `z-claw-providers` | reqwest-based 流式 API 客户端 |
| `runtime::session` | `z-claw-agent` | JSONL 持久化、compaction、fork |
| `runtime::permissions` | `z-claw-security` | PermissionPolicy + allow/deny/ask 规则引擎 |
| `runtime::mcp_stdio` | `z-claw-tools::mcp` | 子进程 MCP 通信 |
| `tools` | `z-claw-tools` | 内置工具（shell/文件/搜索） |
| `plugins` | `z-claw-skills` | 插件/技能系统 |
| `rusty-claude-cli` | `apps/z-claw` | CLI 二进制入口 |
| `claw-analog` | (未来 Phase) | 轻量 CI/script Agent |

---

## 5. 技能系统（借鉴 Claude Code + Hermes Agent）

### SKILL.md 格式（标准）

采用 Claude Code 的 `SKILL.md` frontmatter 格式，兼容 agentskills.io：

```markdown
---
name: code-review
description: "Review code changes and provide feedback"
when_to_use: "When user requests code review, asks to check changes, or after writing code"
allowed-tools: ["Read", "Grep", "Glob", "Bash"]
model: sonnet              # haiku | sonnet | opus — 按技能复杂度路由
user-invocable: true       # 用户可通过 /code-review 显式调用
paths: ["src/**", "crates/**"]  # 条件激活: 匹配当前操作文件路径时自动注入
---

## Instructions

You are a code reviewer. Focus on: correctness, security, performance, style.
Provide specific, actionable feedback with file paths and line numbers.

## Examples

- User: "review this PR" → analyze git diff, check for security issues
```

### 关键设计点

**三级加载源**（借鉴 Claude Code）：

| 层级      | 路径                     | 说明                            |
|---------|------------------------|-------------------------------|
| User    | `~/.z-claw/skills/`    | 用户全局技能                        |
| Project | `.claw/skills/` (向上遍历) | 项目级技能（受版本控制）                  |
| Bundled | 编译进二进制                 | 内置技能（如 code-review、summarize） |

**四种激活方式**：

1. **显式调用**：用户输入 `/skill-name` 触发
2. **条件路径匹配**：`paths` frontmatter 匹配当前操作文件路径（gitignore-style glob）
3. **关键词触发**：`when_to_use` 描述匹配用户意图（由 LLM 判定）
4. **自动学习**：会话分析发现重复模式自动生成候选技能（借鉴 Hermes Agent）

**Skill trait 设计**：

```rust
// z-claw-skills

#[async_trait]
pub trait Skill: Send + Sync {
    /// Unique skill identifier
    fn name(&self) -> &str;
    /// System prompt augmentation when skill is active
    fn system_prompt_augment(&self) -> &str;
    /// Whether this skill matches the current context
    fn matches(&self, ctx: &SkillContext) -> bool;
    /// Tools this skill is allowed to use
    fn allowed_tools(&self) -> &[String];
    /// Preferred model for this skill (None = use default)
    fn model_override(&self) -> Option<ModelTier>;
}

pub struct SkillRegistry {
    skills: HashMap<String, Arc<dyn Skill>>,
    /// Skills indexed by path pattern for fast matching
    path_index: Vec<(GlobPattern, Vec<String>)>,
    /// Auto-learned candidate skills
    learned: Vec<SkillCandidate>,
}
```

**条件路径激活**（借鉴 Claude Code）：

```rust
impl SkillRegistry {
    /// Discover skills whose `paths` pattern matches the given file paths.
    /// Called at turn start to inject relevant skills into the system prompt.
    pub fn activate_for_paths(&self, paths: &[PathBuf]) -> Vec<Arc<dyn Skill>> {
        self.path_index.iter()
            .filter(|(pattern, _)| paths.iter().any(|p| pattern.matches(p)))
            .flat_map(|(_, skill_ids)| skill_ids.iter())
            .filter_map(|id| self.skills.get(id).cloned())
            .collect()
    }
}
```

---

## 6. Provider 路由与故障转移（借鉴 Pi + Claude Code）

### 模型层级与按复杂度路由

```rust
// z-claw-providers::routing

/// Model capability tier — maps to Claude Code's Haiku/Sonnet/Opus routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// Simple tasks: summarization, classification, trivial edits
    /// → local Ollama / Haiku / GPT-4o-mini (fast, cheap)
    Simple,
    /// Medium tasks: code generation, refactoring, debugging
    /// → Sonnet / GPT-4o (balanced)
    Medium,
    /// Complex tasks: architectural design, Plan mode, deep reasoning
    /// → Opus / Claude Opus  (maximum reasoning)
    Complex,
}

pub struct ProviderChain {
    primary: Arc<dyn LlmProvider>,
    fallbacks: Vec<Arc<dyn LlmProvider>>,
    router: ModelRouter,
}

pub struct ModelRouter {
    rules: Vec<RoutingRule>,
}

pub struct RoutingRule {
    pub tier: ModelTier,
    pub provider_id: String,
    pub model_id: String,
}
```

### 自动模型选择策略

```
用户消息 → 复杂度评估 → ModelTier → Router 选择 provider+model
                                          │
  Skill "model: opus" ─────────────────────┤ (skill 级别的 model override)
  Plan mode ───────────────────────────────┤ (plan mode 自动升级到 Complex)
  /model 命令 ─────────────────────────────┤ (用户显式覆盖，最高优先级)
  配置文件 routing.complexity_rules ────────┤
```

### 1M Context 后缀机制（借鉴 Claude Code）

```rust
/// Model identifier with optional 1M context support.
/// When the user/provider supports 1M context, append `[1m]` suffix.
/// Example: "claude-sonnet-4-6" → "claude-sonnet-4-6[1m]"
pub struct ModelId {
    pub base: String,
    pub extended_context: bool,  // true → append [1m] suffix at API call time
}
```

故障转移策略：

1. 尝试 `primary` → 失败则尝试 `fallbacks[0]` → `fallbacks[1]` → ...
2. 每个 provider 重试 2 次（指数退避: 1s, 2s, 4s）
3. 所有 provider 失败后返回 `ClawError::AllProvidersFailed`
4. 故障 provider 标记为降级，60s 冷却期后恢复探测

---

## 7. 记忆系统（SQLite + FTS5 + Embeddings）

### 四类型记忆 Taxonomy（借鉴 Claude Code）

| 类型          | 用途             | 生命周期 | 示例                                           |
|-------------|----------------|------|----------------------------------------------|
| `user`      | 用户角色、偏好、知识背景   | 长期   | "用户是 Rust 后端工程师，偏好函数式风格"                     |
| `feedback`  | 从错误/成功中提炼的行为规则 | 长期   | "不要为不可能发生的状态添加错误处理"                          |
| `project`   | 非代码可推知的项目上下文   | 中期   | "2026-Q2 目标：降低首字节延迟到 <100ms"                 |
| `reference` | 外部系统指针         | 长期   | "Grafana 面板: grafana.internal/d/api-latency" |

### 双层存储结构

```
~/.z-claw/memory/
  MEMORY.md             # 索引文件（只含链接，≤200 行）
  user_role.md           # 用户记忆文件
  feedback_testing.md    # 反馈记忆文件
  project_goals.md       # 项目记忆文件
  ...
```

**MEMORY.md 索引格式**（每行一条链接）：

```markdown
- [User role](user_role.md) — Rust 后端工程师，偏好函数式风格
- [Feedback: testing](feedback_testing.md) — 集成测试必须连接真实 DB
```

**记忆文件格式**（带 frontmatter）：

```markdown
---
name: user-role
description: 用户角色和技术偏好
type: user
---

Content...
```

### MemoryBackend trait

```rust
// z-claw-memory

#[async_trait]
pub trait MemoryBackend: Send + Sync {
    // Session messages
    async fn append_message(&self, session_id: &str, role: &str, content: &str) -> Result<()>;
    async fn load_recent(&self, session_id: &str, limit: usize) -> Result<Vec<HistoryMessage>>;

    // Knowledge (FTS5 full-text search)
    async fn store_knowledge(&self, entry: KnowledgeEntry) -> Result<String>;
    async fn forget_knowledge(&self, id: &str) -> Result<bool>;
    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<KnowledgeEntry>>;

    // Semantic search (embeddings)
    async fn embed_and_store(&self, text: &str, metadata: Value) -> Result<String>;
    async fn semantic_search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;

    // Episodic memory (compacted summaries)
    async fn compact_session(&self, session_id: &str, summary: &str) -> Result<()>;
    async fn recall_context(&self, query: &str, budget_tokens: u32) -> Result<String>;

    // Typed memory (new — Claude Code style)
    async fn load_memories_by_type(&self, mem_type: MemoryType) -> Result<Vec<MemoryEntry>>;
    async fn save_memory(&self, entry: MemoryEntry) -> Result<()>;
    async fn delete_memory(&self, id: &str) -> Result<()>;
    async fn update_memory_index(&self) -> Result<()>;
}
```

### 自动记忆提取（借鉴 Claude Code）

在每轮对话结束时，触发后台 `auto_memory` 任务：

1. 分析本轮对话的关键信息（用户偏好、反馈、项目决策）
2. 分类到四类型 taxonomy
3. 写入对应的记忆文件
4. 更新 MEMORY.md 索引

数据库文件：

- `$DATA_DIR/z-claw/sessions.db` — sessions + messages
- `$DATA_DIR/z-claw/knowledge.db` — knowledge + FTS5 + episodic
- `$DATA_DIR/z-claw/embeddings/` — vector index（sqlite-vec 或 lance）

---

## 8. GPUI UI 架构（借鉴 Zed）

### 8.1 Entity-Context 核心模式

GPUI 的核心是 **Entity-Context-Render** 模式（非传统 MVC）：

```
Entity<T>  ─── 封装的不可变状态单元
  │
  ├── entity.read(cx)    → &T (不可变读取)
  ├── entity.update(cx, |this, cx| { ... })  → &mut T (可变写入 + 触发重绘)
  └── entity.downgrade() → WeakEntity<T> (异步闭包中的弱引用)
```

**关键原则**：

- 每个有独立生命周期的状态对象使用 `Entity<T>` 封装
- 异步闭包中传递 `WeakEntity<T>` 而非 `Entity<T>`，防止已丢弃对象 panic
- `cx.notify()` 标记需要重新渲染，`cx.emit(event)` 向上冒泡事件

### 8.2 Panel/Dock 停靠系统

借鉴 Zed 的 `Panel` trait 实现左/右侧边栏系统：

```rust
// z-claw-ui::panel

pub trait Panel: Focusable + EventEmitter<PanelEvent> + Render {
    /// Unique persistent name for layout serialization
    fn persistent_name() -> &'static str;
    /// Left or Right dock
    fn position(&self, window: &Window, cx: &App) -> DockPosition;
    /// Default width in pixels
    fn default_size(&self, window: &Window, cx: &App) -> Pixels;
    /// Icon shown in the dock toggle button
    fn icon(&self, window: &Window, cx: &App) -> Option<IconName>;
    /// Action dispatched to toggle this panel
    fn toggle_action(&self) -> Box<dyn Action>;
}

pub enum DockPosition { Left, Right }
```

**内建 Panel**：

| Panel          | 位置 | 内容                  |
|----------------|----|---------------------|
| `ChatPanel`    | 中心 | 主对话视图               |
| `SessionPanel` | 左侧 | 会话列表、搜索、新建/删除       |
| `AgentPanel`   | 右侧 | Agent 管理、子 Agent 状态 |
| `SkillsPanel`  | 右侧 | 已安装技能、技能市场          |

### 8.3 PaneGroup 标签页布局

借鉴 Zed 的 `PaneGroup` 树形布局实现可分割工作区：

```rust
pub struct PaneGroup {
    pub root: Member,
}

pub enum Member {
    Pane(Entity<Pane>),          // 单个标签页组
    Axis(PaneAxis),              // 水平或垂直分割
}

pub struct PaneAxis {
    pub axis: Axis,              // Horizontal | Vertical
    pub children: Vec<Member>,
}
```

### 8.4 视图层级

```
AppModel (Entity<AppModel>)
  │
  ├── Workspace (PaneGroup 树形布局)
  │   ├── Pane: Chat    → ChatView
  │   │   ├── MessageList（虚拟滚动）
  │   │   │   └── MessageBubble
  │   │   │       ├── RoleBadge（用户/助手/工具）
  │   │   │       ├── MarkdownBody（渲染后的富文本）
  │   │   │       ├── CodeBlock（语法标签 + 复制按钮）
  │   │   │       └── ToolCallCard（可折叠，显示运行状态/结果）
  │   │   ├── TypingIndicator（流式输出时闪烁光标）
  │   │   └── MessageInput（多行输入 + 发送按钮）
  │   │
  │   ├── Pane: Agents  → AgentMgmtView
  │   ├── Pane: Skills  → SkillsView
  │   └── Pane: Diff    → DiffView
  │
  ├── Dock::Left (SessionPanel)
  │   ├── SessionList（会话列表 + 搜索 + 重命名/删除）
  │   └── NewSession 按钮
  │
  ├── Dock::Right (AgentPanel / SkillsPanel)
  │   ├── Agent 管理页
  │   ├── 技能已安装/市场
  │   └── 子 Agent 运行状态
  │
  ├── SettingsPanel（Modal）
  │   ├── Provider 配置页
  │   ├── 模型选择 + 路由规则
  │   ├── 安全策略配置
  │   ├── MCP 服务器管理
  │   └── 技能管理
  │
  ├── PlanModePanel（Modal）
  │   ├── 计划文本编辑区
  │   ├── 步骤确认 checkbox 列表
  │   └── 批准/修改 按钮
  │
  └── ApprovalDialog（Modal）
      ├── 工具名 + 参数预览
      ├── 安全等级标识
      └── 批准/拒绝 按钮（带倒计时）
```

### 8.5 主题系统

借鉴 Zed 的 `ThemeColors` 语义色系统（~200 颜色定义）：

```rust
pub struct ThemeColors {
    // Border
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,

    // Surface
    pub surface_background: Hsla,
    pub background: Hsla,
    pub elevated_surface_background: Hsla,

    // Element (button, input, etc.)
    pub element_background: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,

    // Ghost element (transparent background variant)
    pub ghost_element_background: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,

    // Text
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub text_disabled: Hsla,

    // Icon
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_accent: Hsla,

    // Panel / Tab bar
    pub panel_background: Hsla,
    pub tab_bar_background: Hsla,
    pub tab_active_background: Hsla,
    pub tab_inactive_background: Hsla,

    // Status bar / Title bar
    pub status_bar_background: Hsla,
    pub title_bar_background: Hsla,
}
```

所有颜色使用 `Hsla`（色相/饱和度/亮度/透明度），通过 `cx.theme().colors().xxx` 访问。间距使用 `DynamicSpacing` 枚举（Base01
紧凑 → Base12 舒适）。

### 8.6 组件库模式

借鉴 Zed `ui` crate 的组件设计：

```rust
// Builder pattern — 所有组件采用链式构建
#[derive(IntoElement)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    start_icon: Option<Icon>,
    end_icon: Option<Icon>,
    key_binding: Option<KeyBinding>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self { ... }
    pub fn start_icon(mut self, icon: Icon) -> Self { ... }
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self { ... }
}

// 使用示例
Button::new("send-btn", "Send")
.start_icon(Icon::new(IconName::Send))
.on_click( | event, window, cx| { ... })
```

**计划内建的组件**：

| 组件                      | 说明              |
|-------------------------|-----------------|
| `Button` / `IconButton` | 基础按钮、图标按钮       |
| `Tab` / `TabBar`        | 标签页组件           |
| `ContextMenu`           | 右键菜单            |
| `Popover` / `Tooltip`   | 悬浮卡片/提示         |
| `Modal`                 | 模态对话框           |
| `List`                  | 可滚动列表（虚拟滚动）     |
| `Disclosure`            | 展开/折叠面板         |
| `Toggle`                | 开关切换            |
| `Chip`                  | 标签（如安全等级 Badge） |
| `MessageBubble`         | 对话消息气泡          |
| `CodeBlock`             | 代码块（语法高亮 + 复制）  |
| `ToolCallCard`          | 工具调用卡片（可折叠）     |

### 8.7 Keybinding 快捷键系统

借鉴 Zed 的上下文链 + JSON 配置：

```json
{
  "context": "ChatPanel",
  "bindings": {
    "ctrl-enter": "chat::SendMessage",
    "ctrl-n": "session::NewSession",
    "ctrl-shift-c": "chat::CopyLastResponse",
    "ctrl-shift-l": "panel::ToggleSidebar"
  }
}
```

上下文链用 `>` 分隔（如 `"ChatPanel > MessageInput"`），支持键盘 chord（如 `"ctrl-k ctrl-o"`）。

```rust
// Action 定义
#[derive(Clone, PartialEq, Debug, Deserialize, Action)]
#[action(namespace = "chat")]
pub struct SendMessage;

// 在 View 中注册
impl Render for ChatView {
    fn render(&mut self, window: &mut Window, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("ChatPanel")
            .on_action(cx.listener(|this, action: &SendMessage, window, cx| {
                this.send_current_message(window, cx);
            }))
    }
}
```

### 8.8 GPUI 异步模式

借鉴 Zed 的 spawn 模式：

```rust
// z-claw-ui::app

pub struct AppModel {
    agent_loop: AgentLoop,
    event_rx: mpsc::UnboundedReceiver<AgentEvent>,
    event_tx: mpsc::UnboundedSender<AgentEvent>,
    messages: Vec<MessageItem>,
    streaming: Option<StreamingState>,
    sessions: Vec<SessionSummary>,
    active_session_id: String,
    pending_approval: Option<ApprovalRequest>,
    task_registry: TaskRegistry,    // 统一任务管理
}

impl AppModel {
    /// Send user message — spawns agent run in background
    fn send_message(&mut self, content: &str, cx: &mut ModelContext<Self>) {
        let content = content.to_string();
        let mut agent = self.agent_loop.clone();
        let event_tx = self.event_tx.clone();

        cx.spawn(|this, mut cx| async move {
            let result = agent.run_turn(&content, &event_tx).await;
            this.update(&mut cx, |this, cx| {
                this.streaming = None;
                cx.notify();  // trigger re-render
            }).ok();
        }).detach();  // fire-and-forget
    }

    /// Poll agent events (called on every frame via cx.on_frame)
    fn poll_events(&mut self, cx: &mut ModelContext<Self>) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                AgentEvent::TextDelta(delta) => {
                    self.apply_delta(&delta);
                    cx.notify();
                }
                AgentEvent::ToolCallStarted { name } => {
                    self.add_tool_card(&name);
                    cx.notify();
                }
                AgentEvent::ApprovalRequired { tool_name, arguments } => {
                    self.pending_approval = Some(ApprovalRequest { tool_name, arguments });
                    cx.notify();
                }
                AgentEvent::TaskCreated { task_id, task_type } => {
                    self.task_registry.register(task_id, task_type);
                }
                AgentEvent::TaskCompleted { task_id, summary } => {
                    self.task_registry.complete(task_id, summary);
                }
            }
        }
    }
}
```

**GPUI 异步关键模式**：

| 方法                                                           | 用途                     |
|--------------------------------------------------------------|------------------------|
| `cx.spawn(\|this, cx\| async { ... }).detach()`              | 后台异步任务，fire-and-forget |
| `cx.spawn_in(window, \|this, cx\| async { ... })`            | 需要 window context 的异步  |
| `cx.defer_in(window, \|this, window, cx\| { ... })`          | 延迟到当前渲染周期后执行           |
| `cx.observe_global::<SettingsStore>(\|cx\| { ... })`         | 响应全局设置变化               |
| `cx.observe_new(\|workspace, window, cx\| { ... })`          | 观察新 Entity 创建          |
| `cx.subscribe(&entity, \|this, entity, event, cx\| { ... })` | 订阅 Entity 事件           |
| `entity.downgrade()` / `weak.upgrade()`                      | 异步闭包中的弱引用安全模式          |

---

## 9. 平台抽象层

```rust
// z-claw-core::platform

pub trait Platform: Send + Sync {
    fn shell(&self) -> &dyn ShellExecutor;
    fn filesystem(&self) -> &dyn FileSystem;
    fn network(&self) -> &dyn Network;
    fn process(&self) -> &dyn ProcessManager;
    fn data_dir(&self) -> PathBuf;
    fn config_dir(&self) -> PathBuf;
    fn os_info(&self) -> OsInfo;
}

pub struct NativePlatform;  // Real OS implementation

#[cfg(test)]
pub struct TestPlatform;    // Mock for testing
```

---

## 10. 配置系统（TOML + 五级配置层）

### 五级配置层次（借鉴 Claude Code）

优先级从低到高：

| 级别      | 路径                          | 说明       | 版本控制       |
|---------|-----------------------------|----------|------------|
| User    | `~/.z-claw/settings.json`   | 用户全局设置   | 否          |
| Project | `.claw/settings.json`       | 项目共享设置   | 是          |
| Local   | `.claw/settings.local.json` | 项目本地设置   | gitignored |
| Flag    | `--settings` 参数             | 命令行覆盖    | 否          |
| Policy  | `managed-settings.json`     | 企业策略（只读） | 否          |

合并策略：按优先级逐层 merge，高优先级覆盖低优先级。

### 主配置文件

位置: `~/.z-claw/config.toml`

```toml
[providers.anthropic]
base_url = "https://api.anthropic.com/v1"
api_key_env = "ANTHROPIC_API_KEY"
default_model = "claude-sonnet-4-20250514"

[providers.openai]
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"
default_model = "gpt-4o"

[providers.ollama]
base_url = "http://localhost:11434/v1"
api_key = "ollama"
default_model = "llama3"

[routing]
default_provider = "anthropic"
fallback_chain = ["openai", "ollama"]

[routing.complexity_rules]
simple = "ollama"       # 简单任务用本地模型
medium = "openai"        # 中等任务用便宜云端
complex = "anthropic"    # 复杂任务用最强模型

[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "."]
lazy = false

[mcp_servers.github]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-github"]
lazy = true

[policy]
blocked_commands = ["rm -rf /", "format c:"]
allowed_paths = ["~"]
default_security_level = "confirm_execute"

[policy.sandbox]
enabled = true
provider = "bubblewrap"  # or "docker", "landlock", "seatbelt"

[memory]
compaction_threshold = 50
compaction_keep_recent = 10
embedding_model = "all-MiniLM-L6-v2"
semantic_search_enabled = true
```

---

## 11. Hooks 钩子系统（借鉴 Claude Code）

### Hook 事件（Agent 生命周期）

| 事件                   | 触发时机     | 用途                 |
|----------------------|----------|--------------------|
| `PreToolUse`         | 工具执行前    | 验证参数、拦截危险操作、注入环境变量 |
| `PostToolUse`        | 工具执行后    | 格式化输出、自动格式化代码、触发通知 |
| `PostToolUseFailure` | 工具执行失败   | 错误上报、重试逻辑          |
| `UserPromptSubmit`   | 用户提交输入   | 输入预处理、敏感信息过滤       |
| `SessionStart`       | 会话开始     | 初始化环境、加载项目上下文      |
| `SessionEnd`         | 会话结束     | 清理临时文件、保存状态        |
| `Stop`               | Agent 停止 | 最终验证、摘要生成          |
| `PreCompact`         | 上下文压缩前   | 保存关键上下文            |
| `PostCompact`        | 上下文压缩后   | 验证压缩质量             |
| `PermissionRequest`  | 权限请求时    | 自定义权限策略            |
| `Notification`       | 通知事件     | 桌面通知、Slack/邮件集成    |
| `CwdChanged`         | 工作目录变化   | 自动加载对应项目配置         |

### Hook 类型（4 种）

```rust
// z-claw-agent::hooks

pub enum HookType {
    /// Execute a shell command
    Command {
        command: String,
        timeout_ms: u64,
        async_mode: bool,      // true = non-blocking
    },
    /// Send HTTP POST webhook
    Http {
        url: String,
        headers: HashMap<String, String>,
    },
    /// Evaluate with an LLM
    Prompt {
        prompt: String,
        model: Option<ModelTier>,
    },
    /// Delegate to a sub-agent for verification
    Agent {
        agent_id: String,
    },
}

pub struct HookDefinition {
    pub event: HookEvent,
    pub hook_type: HookType,
    /// Optional — only execute when tool name matches
    pub matcher: Option<String>,
    /// Optional — only execute when condition is true
    pub condition: Option<String>,
}
```

### Hook 配置格式

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "prettier --write \"$CLaw_FILE_PATH\"",
            "async": true
          }
        ]
      }
    ],
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "cat .claw/CLAW.md 2>/dev/null",
            "timeout_ms": 1000
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "https://hooks.slack.com/...",
            "headers": {}
          }
        ]
      }
    ]
  }
}
```

### Hook 输出协议

Hook 的 stdout 输出解析为 JSON 控制协议：

```json
{
  "continue": true,
  // false → 阻止后续执行
  "stopReason": "File is read-only",
  "suppressOutput": true,
  // 隐藏工具输出
  "decision": "block"
}
```

### 实现策略

- Phase 1-2: 仅实现 `PreToolUse` + `PostToolUse` command 类型（最关键的安全控制点）
- Phase 3-4: 扩展全部 12 种 event + 4 种 type
- Hook 引擎独立于 Agent Loop，通过事件总线通信

---

## 12. Plan Mode 计划模式（借鉴 Claude Code）

### 工作流

```
用户请求复杂任务
  │
  ▼
EnterPlanMode() → permission mode 切换为 "plan"
  │  (只读工具: Read/Grep/Glob/Explore)
  │
  ▼
探索阶段: Agent 分析代码库、确认方案
  │  (可用 AskUserQuestion 澄清需求)
  │
  ▼
ExitPlanMode(plan_text) → 展示 plan 给用户
  │  (用户可确认/修改/拒绝)
  │
  ▼
用户批准 → permission mode 恢复为 "execute"
  │
  ▼
VerifyPlanExecution() → 验证 plan 被正确执行
```

### 核心类型

```rust
// z-claw-agent::plan

pub enum AgentMode {
    /// Full access — all tools available (with security gates)
    Execute,
    /// Read-only — only read/search tools, no writes, no shell
    Plan,
}

pub struct Plan {
    pub id: String,
    pub title: String,
    pub steps: Vec<PlanStep>,
    pub status: PlanStatus,
    pub created_at: DateTime<Utc>,
}

pub struct PlanStep {
    pub description: String,
    pub status: StepStatus,       // pending | in_progress | completed | skipped
    pub verification: Option<String>,
}

pub enum PlanStatus {
    Draft,
    AwaitingApproval,
    Approved,
    Executing,
    Completed,
    Rejected,
}
```

### Plan Mode 中的模型路由

Plan mode 自动升级模型复杂度：

- 默认使用 Sonnet → Plan mode 自动升级为 Opus（`opusplan` alias）
- Haiku 用户在 Plan mode 中自动升级为 Sonnet
- Plan mode 中的 `cx.spawn` 仅允许只读工具

### 实现策略

- Phase 1: 暂不实现（MVP 仅需基础对话）
- Phase 2-3: 实现 Enter/Exit Plan Mode + 只读 permission 切换
- Phase 5: 完整 Plan Mode 体验（步骤追踪、自动验证）

---

## 13. 实施路线图

### Phase 1: MVP（2-3 周）— 本地聊天跑通

**Crate**: z-claw-core, z-claw-providers, z-claw-agent, z-claw-tools, z-claw-config, z-claw-ui, apps/z-claw

- [ ] `z-claw-core`: 全部 trait + 类型定义
- [ ] `z-claw-config`: TOML 加载（单 provider）
- [ ] `z-claw-providers`: Ollama provider（本地优先，零配置即可运行）
- [ ] `z-claw-agent`: 基础 Agent Loop（双循环、流式、事件发射）
- [ ] `z-claw-agent`: PreToolUse/PostToolUse hook 事件总线（最小实现）
- [ ] `z-claw-tools`: `read_file`, `write_file`, `list_directory`, `execute_command`
- [ ] `z-claw-ui`: GPUI 窗口 + Entity-Context 模式 + ChatView（消息列表、文本输入、流式更新、Markdown 渲染）
- [ ] `z-claw-ui`: 主题系统（ThemeColors 语义色表） + 基础组件库（Button/Tab/Modal）
- [ ] `z-claw-ui`: Sidebar（会话列表、新建/删除）
- [ ] `apps/z-claw`: `main.rs` 启动流程
- [ ] SQLite 基础持久化（sessions + messages）

**里程碑**: 用户输入 → 流式回复 → 模型可读写文件

### Phase 2: 多供应商 + 安全（2 周）

**Crate**: z-claw-security, z-claw-providers（扩展）

- [ ] `z-claw-providers`: Anthropic, OpenAI, DeepSeek 适配器
- [ ] `z-claw-providers`: ProviderChain 故障转移 + ModelRouter 按复杂度路由（ModelTier 三级路由）
- [ ] `z-claw-security`: 5 级安全分级 + 审批流
- [ ] `z-claw-ui`: ApprovalDialog 审批对话框
- [ ] `z-claw-ui`: SettingsPanel（provider 配置 + 安全策略 + 五级配置）
- [ ] `z-claw-ui`: ToolCallCard 工具调用状态卡片
- [ ] `z-claw-ui`: Keybinding 系统（JSON 配置 + 上下文链）
- [ ] `z-claw-agent`: Plan Mode 基础（read-only permission 切换）

**里程碑**: 多供应商自动故障转移，危险操作需审批，基础 Plan Mode

### Phase 3: 工具生态 + MCP + Hooks（2 周）

**Crate**: z-claw-tools（扩展）, z-claw-agent（hooks 引擎）

- [ ] `z-claw-tools`: HTTP 请求工具
- [ ] `z-claw-tools`: MCP 客户端（rmcp）+ McpPool
- [ ] `z-claw-tools`: 浏览器自动化工具
- [ ] `z-claw-agent`: Hooks 执行引擎（4 种 hook type: Command/Http/Prompt/Agent）
- [ ] `z-claw-agent`: 完整 12 种 hook event 生命周期
- [ ] `z-claw-ui`: MCP 服务器管理界面
- [ ] `z-claw-ui`: Hooks 配置界面
- [ ] 工具审批策略细化（per-tool 配置）

**里程碑**: MCP 服务器即插即用，Hook 系统可用，工具生态可扩展

### Phase 4: 记忆系统（2 周）

**Crate**: z-claw-memory

- [ ] `z-claw-memory`: 四类型记忆 taxonomy（user/feedback/project/reference）
- [ ] `z-claw-memory`: FTS5 全文搜索
- [ ] `z-claw-memory`: 向量嵌入 + 语义搜索（sqlite-vec 或 lance）
- [ ] `z-claw-memory`: 会话自动压缩（LLM 摘要）
- [ ] `z-claw-memory`: MEMORY.md 索引 + 话题文件双层存储
- [ ] `z-claw-memory`: RAG 检索增强生成
- [ ] `z-claw-agent`: 自动记忆提取（auto-memory 后台任务）
- [ ] `z-claw-tools`: `store_knowledge`, `forget_knowledge`, `search_memory`
- [ ] `z-claw-ui`: 记忆浏览/搜索界面

**里程碑**: Agent 可跨会话记忆和检索知识

### Phase 5: 技能系统 + Plan Mode（2 周）

**Crate**: z-claw-skills, z-claw-agent（plan 扩展）

- [ ] `z-claw-skills`: SKILL.md frontmatter 加载器（三级加载源: user/project/bundled）
- [ ] `z-claw-skills`: 条件路径激活（`paths` pattern matching）+ SkillRegistry
- [ ] `z-claw-skills`: 会话自动学习候选技能
- [ ] `z-claw-ui`: SkillsView（技能市场/已安装/创建向导）
- [ ] `z-claw-agent`: 完整 Plan Mode（Enter/Exit/Verify 三步 + 步骤追踪）
- [ ] `z-claw-ui`: PlanModePanel（计划文本编辑 + 步骤确认）
- [ ] 社区技能 Hub 集成 / agentskills.io 兼容导出

**里程碑**: 技能可安装、可创建、可分享，Plan Mode 完整可用

### Phase 6: 高级功能（2+ 周）

- [ ] 子 Agent 管理（隔离执行、前景/后台模式、worktree 隔离）
- [ ] Task 统一抽象层（任务注册、生命周期、progress 汇报）
- [ ] Cron 调度器（定时任务 + 通知）
- [ ] 系统托盘 + 后台运行
- [ ] OS 原生通知（Notification hook）
- [ ] Code Diff 查看器
- [ ] Panel/Dock 停靠系统（PaneGroup 树形可分割布局）
- [ ] 配置文件热重载
- [ ] 多窗口支持
- [ ] 平台沙箱集成（Windows: Sandboxie, macOS: Seatbelt, Linux: Bubblewrap）
- [ ] Skill-level model routing（Skill 指定使用 Opus/Sonnet/Haiku）

---

## 14. 文件布局总览

```
z-claw/
├── Cargo.toml
├── apps/z-claw/src/
│   └── main.rs
├── crates/
│   ├── z-claw-core/src/
│   │   ├── lib.rs
│   │   ├── error.rs
│   │   ├── types.rs          # Session, HistoryMessage, SessionSummary
│   │   ├── event.rs          # AgentEvent enum
│   │   └── platform.rs       # Platform trait
│   │
│   ├── z-claw-agent/src/
│   │   ├── lib.rs
│   │   ├── loop_.rs          # AgentLoop::run_turn()
│   │   ├── harness.rs        # Harness: providers + tools + skills + memory
│   │   ├── session.rs        # Session state + context compression
│   │   ├── subagent.rs       # SubAgent manager
│   │   ├── context.rs        # Context builder + auto-memory
│   │   ├── hooks.rs          # Hook engine (12 events, 4 types)
│   │   ├── plan.rs           # Plan mode (Enter/Exit/Verify)
│   │   └── task.rs           # Task abstraction (register/lifecycle/progress)
│   │
│   ├── z-claw-providers/src/
│   │   ├── lib.rs            # LlmProvider trait + StreamChunk types
│   │   ├── anthropic.rs
│   │   ├── openai.rs
│   │   ├── google.rs
│   │   ├── deepseek.rs
│   │   ├── ollama.rs
│   │   └── routing.rs        # ProviderChain, ModelRouter, ModelTier
│   │
│   ├── z-claw-tools/src/
│   │   ├── lib.rs            # Tool trait + ToolRegistry
│   │   ├── shell.rs
│   │   ├── filesystem.rs
│   │   ├── http.rs
│   │   ├── browser.rs
│   │   ├── knowledge.rs
│   │   └── mcp.rs            # McpPool, McpTool adapter
│   │
│   ├── z-claw-skills/src/
│   │   ├── lib.rs            # Skill trait + SkillRegistry + path_index
│   │   ├── loader.rs         # SKILL.md frontmatter loader
│   │   ├── forge.rs          # Skill creation wizard
│   │   └── agentskills.rs    # agentskills.io format
│   │
│   ├── z-claw-memory/src/
│   │   ├── lib.rs            # MemoryBackend trait + MemoryType enum
│   │   ├── sqlite.rs         # SQLite + FTS5
│   │   ├── embeddings.rs     # Vector embeddings
│   │   ├── rag.rs            # RAG retrieval
│   │   └── memdir.rs         # MEMORY.md index + topic file management
│   │
│   ├── z-claw-security/src/
│   │   ├── lib.rs            # SecurityLevel, SecurityPolicy
│   │   ├── sandbox.rs        # Sandbox provider abstraction
│   │   └── audit.rs          # Audit log
│   │
│   ├── z-claw-config/src/
│   │   ├── lib.rs            # Config load + 5-level merge + watch
│   │   └── schema.rs         # TOML schema types
│   │
│   └── z-claw-ui/src/
│       ├── lib.rs
│       ├── app.rs            # AppModel (Entity/Event/Task registry)
│       ├── theme.rs          # ThemeColors (Hsla semantic color table)
│       ├── spacing.rs        # DynamicSpacing enum
│       ├── keybindings.rs    # Keyboard shortcuts (JSON + context chains)
│       ├── panel.rs          # Panel trait + Dock system
│       ├── pane_group.rs     # PaneGroup tree layout
│       ├── views/
│       │   ├── mod.rs
│       │   ├── chat.rs       # ChatView (message list + streaming)
│       │   ├── sidebar.rs    # SessionPanel (left dock)
│       │   ├── settings.rs   # SettingsPanel (modal)
│       │   ├── skills.rs     # SkillsView
│       │   ├── agents.rs     # AgentMgmtView
│       │   ├── plan.rs       # PlanModePanel (plan text + step checklist)
│       │   ├── hooks.rs      # Hooks config panel
│       │   └── diff.rs       # Code Diff view
│       └── components/
│           ├── mod.rs
│           ├── button.rs
│           ├── tab.rs
│           ├── tab_bar.rs
│           ├── context_menu.rs
│           ├── popover.rs
│           ├── tooltip.rs
│           ├── modal.rs
│           ├── list.rs
│           ├── disclosure.rs
│           ├── toggle.rs
│           ├── chip.rs
│           ├── message_bubble.rs
│           ├── message_input.rs
│           ├── markdown.rs
│           ├── code_block.rs
│           ├── typing_indicator.rs
│           └── tool_call_card.rs
```

---

## 15. 关键技术依赖

| Crate              | Dependencies                                                                  |
|--------------------|-------------------------------------------------------------------------------|
| `z-claw-core`      | serde, serde_json, async-trait, futures, thiserror, uuid                      |
| `z-claw-agent`     | z-claw-core, tokio (rt, sync, time)                                           |
| `z-claw-providers` | z-claw-core, reqwest (rustls-tls, stream, json), tokio                        |
| `z-claw-tools`     | z-claw-core, z-claw-security, tokio, rmcp (client, transport)                 |
| `z-claw-skills`    | z-claw-core, serde_yaml, glob                                                 |
| `z-claw-memory`    | z-claw-core, rusqlite (bundled, vtab), sqlite-vec                             |
| `z-claw-security`  | z-claw-core, serde                                                            |
| `z-claw-config`    | z-claw-core, serde, toml, notify, dirs                                        |
| `z-claw-ui`        | z-claw-core, z-claw-agent, gpui, gpui::gpui_winit, syntect (syntax highlight) |
| `apps/z-claw`      | all crates, gpui, tokio, tracing, tracing-subscriber                          |

---

## 16. 风险与缓解

| 风险                             | 缓解                                                             |
|--------------------------------|----------------------------------------------------------------|
| GPUI 独立使用复杂，依赖 Zed 内部 crate    | MVP 先跑通 hello_world + 基础渲染，逐步引入                                |
| GPUI 的 Entity-Context 异步模式学习曲线 | 严格遵循 Zed 源码模式，先用 `cx.spawn().detach()` 最简单的模式                  |
| Markdown 渲染无现成方案               | 自建轻量解析器，Phase 3+ 考虑集成 comrak                                   |
| GPUI 文档/示例较少                   | 直接参考 Zed 源码（`.ref/zed/crates/agent_ui/`、`.ref/zed/crates/ui/`） |
| Windows 沙箱方案有限                 | Phase 6 之前使用路径白名单 + 权限控制替代                                     |
| Ollama 流式 API 不稳定              | 实现 ProviderChain 自动故障转移                                        |
| Hooks 执行安全问题（第三方 hook 注入）      | Hook 运行在安全沙箱中，敏感操作需用户确认；Policy 级 hook 只读                       |
| 内存/技能文件格式向前兼容                  | 使用 frontmatter 版本号 + schema 校验，丢弃未知字段                          |
| GPUI 在 Windows 平台的稳定性          | MVP 期间在 Windows 上进行充分测试，macOS/Linux 作为次要目标                     |
