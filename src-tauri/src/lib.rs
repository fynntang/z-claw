//! ZClaw Desktop - AI Assistant powered by ZeroClaw
//!
//! 通过嵌入 zeroclaw 二进制并以 Gateway HTTP API 连接，不再依赖 zeroclaw 库。

mod config_bridge;
mod gateway;

use gateway::{api_config_get, api_config_put, api_status, api_tools};
use gateway::{
    spawn_gateway_sidecar, wait_for_health, fetch_paircode, pair_with_code,
    load_saved_token, save_token, GatewayState, DEFAULT_GATEWAY_PORT,
};
use config_bridge::{toml_to_config_state, merge_config_state_into_toml, ConfigState};
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

/// 应用级状态：Gateway 连接与 token
struct AppGatewayState(Arc<RwLock<GatewayState>>);

/// 启动时：spawn zeroclaw gateway，等待 /health，确保已配对
async fn ensure_gateway_ready(app: &tauri::AppHandle) -> Result<(), String> {
    let port = DEFAULT_GATEWAY_PORT;
    let state = app.state::<AppGatewayState>();
    let g = state.0.write().await;

    // 若未配置使用外部 gateway，则启动 sidecar
    let use_external = std::env::var("ZEROCLAW_USE_EXTERNAL_GATEWAY").unwrap_or_default() == "1";
    if !use_external {
        if let Err(e) = spawn_gateway_sidecar(app, port) {
            return Err(format!("启动 zeroclaw gateway 失败: {}", e));
        }
        g.spawned_by_us.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    drop(g);

    if let Err(e) = wait_for_health(port, std::time::Duration::from_secs(30)).await {
        return Err(format!("等待 gateway 就绪失败: {}", e));
    }

    let g = state.0.write().await;
    if g.token.is_some() {
        if let Some(t) = load_saved_token().map_err(|e| e.to_string())? {
            drop(g);
            let mut g2 = state.0.write().await;
            g2.set_token(t);
            return Ok(());
        }
    }
    drop(g);

    // 需要配对：获取 code -> POST /pair -> 保存 token
    let code = fetch_paircode(port).await.map_err(|e| e.to_string())?;
    let token = pair_with_code(port, &code).await.map_err(|e| e.to_string())?;
    save_token(&token).map_err(|e| e.to_string())?;
    let mut g2 = state.0.write().await;
    g2.set_token(token);
    Ok(())
}

/// 返回当前 Gateway 状态（供 commands 使用）
async fn gateway_state(app: &tauri::AppHandle) -> Result<Arc<RwLock<GatewayState>>, String> {
    app.try_state::<AppGatewayState>()
        .ok_or_else(|| "AppGatewayState 未初始化".to_string())
        .map(|s| s.0.clone())
}

#[tauri::command]
async fn get_status(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let state = gateway_state(&app).await?;
    let gw = state.read().await.clone();
    drop(state);
    api_status(&gw).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn config_get(app: tauri::AppHandle) -> Result<ConfigState, String> {
    let state = gateway_state(&app).await?;
    let gw = state.read().await.clone();
    drop(state);
    let resp = api_config_get(&gw).await.map_err(|e| e.to_string())?;
    toml_to_config_state(&resp.content).map_err(|e| e.to_string())
}

#[tauri::command]
async fn config_set(app: tauri::AppHandle, config: ConfigState) -> Result<(), String> {
    let state = gateway_state(&app).await?;
    let gw = state.read().await.clone();
    drop(state);
    let current = api_config_get(&gw).await.map_err(|e| e.to_string())?;
    let new_toml = merge_config_state_into_toml(&current.content, &config).map_err(|e| e.to_string())?;
    api_config_put(&gw, &new_toml).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn config_validate(config: ConfigState) -> Result<serde_json::Value, String> {
    let mut errors: Vec<String> = Vec::new();
    if config.provider.is_empty() {
        errors.push("provider 不能为空".to_string());
    }
    if config.model.is_empty() {
        errors.push("model 不能为空".to_string());
    }
    if config.api_url.is_empty() && !["ollama", "lmstudio", "llamacpp"].contains(&config.provider.as_str()) {
        errors.push("api_url 不能为空".to_string());
    }
    let valid = errors.is_empty();
    Ok(serde_json::json!({ "valid": valid, "errors": errors }))
}

#[tauri::command]
async fn tools_list(app: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let state = gateway_state(&app).await?;
    let gw = state.read().await.clone();
    drop(state);
    let json = api_tools(&gw).await.map_err(|e| e.to_string())?;
    let arr = json.get("tools").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    Ok(arr)
}

#[tauri::command]
fn providers_list() -> Result<Vec<serde_json::Value>, String> {
    let list = [
        ("openrouter", "OpenRouter", false),
        ("openai", "OpenAI", false),
        ("anthropic", "Anthropic", false),
        ("ollama", "Ollama", true),
        ("deepseek", "DeepSeek", false),
        ("groq", "Groq", false),
    ];
    Ok(list
        .iter()
        .map(|(name, display_name, local)| {
            serde_json::json!({
                "name": name,
                "display_name": display_name,
                "local": local
            })
        })
        .collect())
}

/// 占位：workspace 路径（与 zeroclaw 约定一致）
#[tauri::command]
fn workspace_path() -> Result<String, String> {
    let dir = dirs::config_dir().ok_or("无法获取 config 目录")?;
    Ok(dir.join("zeroclaw").join("workspace").display().to_string())
}

/// 占位：读取 workspace 文件（后续可经 Gateway 或本地文件实现）
#[tauri::command]
async fn workspace_read_file(_relative_path: String) -> Result<String, String> {
    Err("暂未实现：请通过 zeroclaw Gateway 使用".to_string())
}

/// 占位：列出 workspace 目录
#[tauri::command]
async fn workspace_list_dir(_subpath: Option<String>) -> Result<Vec<serde_json::Value>, String> {
    Ok(Vec::new())
}

/// 占位：聊天（后续可经 POST /webhook 或 WebSocket 实现）
#[tauri::command]
async fn chat(
    _message: String,
    _session_id: Option<String>,
    _api_key: String,
    _api_url: String,
    _model: String,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "success": false,
        "content": "",
        "error": "暂未实现：请使用 zeroclaw Web Dashboard 或 WebSocket /ws/chat"
    }))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ZClaw!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let state = Arc::new(RwLock::new(GatewayState::new(DEFAULT_GATEWAY_PORT)));
            app.manage(AppGatewayState(state));
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = ensure_gateway_ready(&app_handle).await {
                    eprintln!("[zclaw] gateway 初始化失败: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_status,
            config_get,
            config_set,
            config_validate,
            tools_list,
            providers_list,
            workspace_path,
            workspace_read_file,
            workspace_list_dir,
            chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZClaw application");
}
