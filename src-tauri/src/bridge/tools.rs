//! Tools bridge for desktop UI.
//!
//! Exposes ZeroClaw tool registry to the frontend.

use serde::{Deserialize, Serialize};

/// Tool info for UI display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}

/// Default tools available in ZeroClaw.
/// These match the tools from zeroclaw::tools::default_tools()
const DEFAULT_TOOLS: &[(&str, &str, &str)] = &[
    (
        "shell",
        "Execute shell commands with security sandboxing",
        "execution",
    ),
    (
        "file_read",
        "Read file contents from the workspace",
        "filesystem",
    ),
    (
        "file_write",
        "Write or create files in the workspace",
        "filesystem",
    ),
    (
        "file_edit",
        "Edit existing files with pattern matching",
        "filesystem",
    ),
    (
        "glob_search",
        "Search for files matching glob patterns",
        "search",
    ),
    (
        "content_search",
        "Search file contents for text patterns",
        "search",
    ),
];

/// Additional tools available with full configuration.
#[allow(dead_code)]
const FULL_TOOLS: &[(&str, &str, &str)] = &[
    // Cron tools
    ("cron_add", "Add a scheduled task", "scheduling"),
    ("cron_list", "List all scheduled tasks", "scheduling"),
    ("cron_remove", "Remove a scheduled task", "scheduling"),
    (
        "cron_run",
        "Manually trigger a scheduled task",
        "scheduling",
    ),
    ("cron_update", "Update a scheduled task", "scheduling"),
    // Memory tools
    (
        "memory_store",
        "Store information in agent memory",
        "memory",
    ),
    (
        "memory_recall",
        "Recall information from agent memory",
        "memory",
    ),
    (
        "memory_forget",
        "Remove information from agent memory",
        "memory",
    ),
    // Git tools
    (
        "git_operations",
        "Perform git operations (status, commit, push, pull)",
        "version-control",
    ),
    // Browser tools
    (
        "browser",
        "Browser automation for web navigation",
        "browser",
    ),
    (
        "browser_open",
        "Open URLs in browser with domain allowlist",
        "browser",
    ),
    // HTTP tools
    (
        "http_request",
        "Make HTTP requests to external APIs",
        "network",
    ),
    (
        "web_fetch",
        "Fetch and extract content from web pages",
        "network",
    ),
    ("web_search", "Search the web for information", "network"),
    // Vision tools
    (
        "screenshot",
        "Capture screenshots of the desktop or windows",
        "vision",
    ),
    (
        "image_info",
        "Get metadata and analysis of images",
        "vision",
    ),
    (
        "pdf_read",
        "Extract text and structure from PDF documents",
        "documents",
    ),
    // Other tools
    (
        "schedule",
        "Schedule one-time or recurring tasks",
        "scheduling",
    ),
    (
        "delegate",
        "Delegate tasks to specialized sub-agents",
        "agent",
    ),
    (
        "pushover",
        "Send push notifications via Pushover",
        "notification",
    ),
    (
        "model_routing_config",
        "Configure model routing rules",
        "config",
    ),
    ("proxy_config", "Configure HTTP proxy settings", "config"),
];

/// Get list of default tools (always available).
pub fn list_tools() -> Vec<ToolInfo> {
    DEFAULT_TOOLS
        .iter()
        .map(|(name, desc, cat)| ToolInfo {
            name: name.to_string(),
            description: desc.to_string(),
            category: cat.to_string(),
        })
        .collect()
}

/// Get list of all available tools (including optional ones).
#[allow(dead_code)]
pub fn list_all_tools() -> Vec<ToolInfo> {
    let mut tools: Vec<ToolInfo> = list_tools();
    tools.extend(FULL_TOOLS.iter().map(|(name, desc, cat)| ToolInfo {
        name: name.to_string(),
        description: desc.to_string(),
        category: cat.to_string(),
    }));
    tools
}

/// Get a specific tool by name.
#[allow(dead_code)]
pub fn get_tool(name: &str) -> Option<ToolInfo> {
    DEFAULT_TOOLS
        .iter()
        .chain(FULL_TOOLS.iter())
        .find(|(n, _, _)| *n == name)
        .map(|(n, desc, cat)| ToolInfo {
            name: n.to_string(),
            description: desc.to_string(),
            category: cat.to_string(),
        })
}
