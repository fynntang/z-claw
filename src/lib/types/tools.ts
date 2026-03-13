/**
 * Tools types for Z-Claw desktop.
 * These types match the Rust ToolInfo structure.
 */

/**
 * Tool information for UI display.
 */
export interface ToolInfo {
	/** Tool name (e.g., "shell", "file_read") */
	name: string;
	/** Human-readable description */
	description: string;
	/** Category for grouping */
	category: string;
}

/**
 * Tool categories for grouping in the UI.
 */
export const TOOL_CATEGORIES = [
	{ id: 'execution', label: '执行', icon: '⚡' },
	{ id: 'filesystem', label: '文件系统', icon: '📁' },
	{ id: 'search', label: '搜索', icon: '🔍' },
	{ id: 'scheduling', label: '调度', icon: '⏰' },
	{ id: 'memory', label: '记忆', icon: '🧠' },
	{ id: 'version-control', label: '版本控制', icon: '📦' },
	{ id: 'browser', label: '浏览器', icon: '🌐' },
	{ id: 'network', label: '网络', icon: '🔗' },
	{ id: 'vision', label: '视觉', icon: '👁️' },
	{ id: 'documents', label: '文档', icon: '📄' },
	{ id: 'agent', label: '智能体', icon: '🤖' },
	{ id: 'notification', label: '通知', icon: '🔔' },
	{ id: 'config', label: '配置', icon: '⚙️' }
] as const;

export type ToolCategory = (typeof TOOL_CATEGORIES)[number]['id'];
