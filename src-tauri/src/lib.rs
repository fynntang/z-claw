//! ZClaw Desktop - AI Assistant powered by ZeroClaw
//!
//! A lightweight AI assistant client built with Tauri.

/// Greet command - example from Tauri template
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ZClaw!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running ZClaw application");
}
