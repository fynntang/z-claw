# 变更记录：frontend-design 技能重设计

**时间**: 2026-03-14  
**类型**: UI 重设计（遵循 frontend-design 技能）

---

## 设计思路（Design Thinking）

- **Purpose**: ZClaw 桌面 AI 助手 — 对话、多 Agent、会话管理。用户需要清晰、高效、有辨识度的界面。
- **Tone**: **Editorial / refined minimal** — 明确层次、单一强调色、充足留白、微妙景深。
- **Differentiation**: 让人记住的一点 — **珊瑚色强调 (#e85d4c)** + **Plus Jakarta Sans** 字体，避免通用 AI 感（无 Inter、无紫渐变）。

---

## 设计系统 (layout.css)

- **字体**: `Plus Jakarta Sans Variable` 为主 UI 字体（技能要求避免 Inter），JetBrains Mono 保留。
- **色彩**: 
  - 背景层级：`--app-bg` #070809、`--app-bg-top` #0a0b0c、`--app-panel` #0d0e10、`--app-surface` #121316、`--app-surface-elevated` #181a1d。
  - 单一强调色：`--app-accent` #e85d4c（珊瑚红），辅以 `--app-accent-dim` / `--app-accent-soft` / `--app-border-focus`。
  - 文字：`--app-text` / `--app-text-secondary` / `--app-text-muted`；分割线：`--app-divider`。
- **背景**: 自上而下线性渐变（`--app-bg-top` → `--app-bg`）营造景深，非纯平。
- **间距**: 引入 `--app-space` / `--app-space-sm` / `--app-space-lg` 便于统一节奏。

---

## 组件级改动

| 组件 | 改动要点 |
|------|----------|
| **TitleBar** | 品牌名使用 accent 色；半透明 + backdrop-blur；`duration-200` 过渡；语义化 `<header>` / `<nav>`。 |
| **AgentSidebar** | 选中态左侧 2px accent 竖条；圆角 `rounded-xl`；focus-visible 环 accent。 |
| **ChannelSidebar** | 标题「对话」小写 + `tracking-widest` 编辑感；选中项左侧 accent 竖条；分隔线区分会话与频道。 |
| **ChatArea** | 空状态主文案加粗（「向 xxx 说点什么」）；助手气泡左侧 2px accent 边框；用户气泡 accent 实色；输入框 focus 环 accent；`leading-relaxed` 行高。 |
| **RightPanel** | 统一使用 `--app-divider` / `--app-surface-elevated`；上传按钮 hover 使用 `--app-accent-soft`；Agent 颜色与主界面一致（rose/amber/emerald）。 |

---

## 可执行与验证

- `pnpm install` 后 `pnpm tauri dev` 可正常运行。
- 无新增 lint 报错；交互状态（hover/focus）统一为 200ms 过渡。

---

## 与 frontend-design 技能对应

- **Typography**: 使用 Plus Jakarta Sans，避免 Inter/Roboto/Arial。
- **Color**: 单一主导深色 + 一个鲜明 accent（珊瑚色），非均匀分布。
- **Backgrounds**: 线性渐变提供景深，非纯色。
- **Spatial**: 留白与间距变量统一；聊天区 max-w-2xl 居中，阅读舒适。
- **Motion**: 统一 `duration-200` 的 transition，高影响处（如 focus）使用 accent 环。
