//! ZClaw Desktop - AI Assistant powered by ZeroClaw
//!
//! A lightweight AI assistant client built with Tauri.

mod agent;
mod bridge;

use agent::AgentResponse;
use bridge::{AppConfig, ChatInput, DesktopBridge, LogEntry, LogLevel, RuntimeStatus, ToolInfo, ValidationResult};

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
            tools_list,
            logs_tail,
            logs_clear
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZClaw application");
}
