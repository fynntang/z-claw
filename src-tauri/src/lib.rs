//! ZClaw Desktop - AI Assistant powered by ZeroClaw
//!
//! A lightweight AI assistant client built with Tauri.

mod agent;
mod bridge;

use agent::AgentResponse;
use bridge::{AppConfig, ChatInput, DesktopBridge, RuntimeStatus, ValidationResult};

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
            config_validate
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZClaw application");
}
