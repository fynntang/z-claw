//! ZClaw Desktop - AI Assistant powered by ZeroClaw
//!
//! A lightweight AI assistant client built with Tauri.

mod agent;
mod bridge;

use agent::AgentResponse;
use bridge::{
    AppConfig, ChatInput, DesktopBridge, LogEntry, LogLevel, RuntimeStatus, ToolInfo, ValidationResult,
    WorkspaceEntry,
};

/// Greet command - example from Tauri template
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ZClaw!", name)
}

/// Chat with the AI agent
#[tauri::command]
async fn chat(
    message: String,
    session_id: Option<String>,
    api_key: Option<String>,
    api_url: Option<String>,
    model: Option<String>,
) -> AgentResponse {
    let input = ChatInput {
        session_id,
        message,
        api_key,
        api_url,
        provider: None,
        model,
    };

    DesktopBridge::chat(input).await
}

/// Get agent status
#[tauri::command]
fn get_status() -> RuntimeStatus {
    DesktopBridge::status()
}

/// Get configuration from ZeroClaw config file
#[tauri::command]
fn config_get() -> Result<AppConfig, String> {
    bridge::config::get_config()
}

/// Save configuration to ZeroClaw config file
#[tauri::command]
fn config_set(config: AppConfig) -> Result<(), String> {
    bridge::config::set_config(config)
}

/// Validate configuration without saving
#[tauri::command]
fn config_validate(config: AppConfig) -> ValidationResult {
    bridge::config::validate_config(&config)
}

#[derive(serde::Serialize)]
struct UIProviderInfo {
    name: String,
    display_name: String,
    local: bool,
}

/// Get list of available AI providers from ZeroClaw
#[tauri::command]
fn providers_list() -> Vec<UIProviderInfo> {
    zeroclaw::providers::list_providers()
        .into_iter()
        .map(|p| UIProviderInfo {
            name: p.name.to_string(),
            display_name: p.display_name.to_string(),
            local: p.local,
        })
        .collect()
}

/// Get list of available tools
#[tauri::command]
fn tools_list() -> Vec<ToolInfo> {
    bridge::tools::list_tools()
}

/// Get recent log entries
#[tauri::command]
async fn logs_tail(limit: Option<usize>, level: Option<String>) -> Vec<LogEntry> {
    let log_level = level.and_then(|l| match l.to_lowercase().as_str() {
        "debug" => Some(LogLevel::Debug),
        "info" => Some(LogLevel::Info),
        "warn" => Some(LogLevel::Warn),
        "error" => Some(LogLevel::Error),
        _ => None,
    });
    bridge::logs::tail_logs(limit, log_level).await
}

/// Clear all logs
#[tauri::command]
async fn logs_clear() {
    bridge::logs::clear_logs().await
}

/// Get ZeroClaw workspace root path (e.g. ~/.config/zeroclaw/workspace).
#[tauri::command]
fn workspace_path() -> String {
    bridge::workspace::workspace_path()
}

/// List directory under workspace. subpath: "" for root, or "memory", "memory/2025-01" etc.
#[tauri::command]
fn workspace_list_dir(subpath: Option<String>) -> Result<Vec<WorkspaceEntry>, String> {
    bridge::workspace::workspace_list_dir(subpath)
}

/// Read a file under workspace (e.g. "IDENTITY.md", "USER.md", "memory/2025-01-01.md").
#[tauri::command]
fn workspace_read_file(relative_path: String) -> Result<String, String> {
    bridge::workspace::workspace_read_file(relative_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            chat,
            get_status,
            config_get,
            config_set,
            config_validate,
            providers_list,
            tools_list,
            logs_tail,
            logs_clear,
            workspace_path,
            workspace_list_dir,
            workspace_read_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZClaw application");
}
