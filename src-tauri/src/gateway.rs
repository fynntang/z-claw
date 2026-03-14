//! Zeroclaw Gateway 进程管理与 HTTP API 客户端。
//! 负责启动 sidecar、等待 /health、配对与 token 持久化、以及 /api/* 请求封装。

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

/// 默认 Gateway 端口（与 zeroclaw 一致）
pub const DEFAULT_GATEWAY_PORT: u16 = 42617;

/// Gateway 基地址
fn base_url(port: u16) -> String {
    format!("http://127.0.0.1:{port}")
}

/// 桌面端持有的 Gateway 状态（端口、token、是否由本进程 spawn）
#[derive(Clone)]
pub struct GatewayState {
    pub port: u16,
    pub token: Option<Arc<str>>,
    pub spawned_by_us: Arc<AtomicBool>,
}

impl GatewayState {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            token: None,
            spawned_by_us: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn base_url(&self) -> String {
        base_url(self.port)
    }

    pub fn set_token(&mut self, t: String) {
        self.token = Some(Arc::from(t.into_boxed_str()));
    }

    pub fn auth_header(&self) -> Option<String> {
        self.token
            .as_ref()
            .map(|t| format!("Bearer {}", t.as_ref()))
    }
}

/// 轮询 GET /health 直到返回 200 或超时
pub async fn wait_for_health(port: u16, timeout: Duration) -> Result<()> {
    let url = format!("{}/health", base_url(port));
    let client = Client::builder().timeout(Duration::from_secs(2)).build()?;
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if let Ok(res) = client.get(&url).send().await {
            if res.status().is_success() {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    anyhow::bail!("gateway /health 未在 {:?} 内就绪", timeout)
}

/// 从应用侧启动 zeroclaw sidecar（gateway 子命令）
pub fn spawn_gateway_sidecar(app: &AppHandle, port: u16) -> Result<()> {
    let shell = app.shell();
    let sidecar = shell
        .sidecar("zeroclaw")
        .map_err(|e| anyhow::anyhow!("zeroclaw sidecar 未找到: {}", e))?;
    sidecar
        .args(["gateway", "--host", "127.0.0.1", "--port", &port.to_string()])
        .spawn()
        .map_err(|e| anyhow::anyhow!("启动 zeroclaw gateway 失败: {}", e))?;
    Ok(())
}

/// 从 GET /admin/paircode 或 POST /admin/paircode/new 获取配对码（localhost）
pub async fn fetch_paircode(port: u16) -> Result<String> {
    let url_new = format!("{}/admin/paircode/new", base_url(port));
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let res = client.post(&url_new).send().await?;
    let status = res.status();
    let body: PaircodeResponse = res.json().await.unwrap_or_default();
    if status.is_success() && !body.code.is_empty() {
        return Ok(body.code);
    }
    let url_get = format!("{}/admin/paircode", base_url(port));
    let res = client.get(&url_get).send().await?;
    let body: PaircodeResponse = res.json().await.unwrap_or_default();
    if !body.code.is_empty() {
        return Ok(body.code);
    }
    anyhow::bail!("无法获取 paircode")
}

#[derive(Deserialize, Default)]
struct PaircodeResponse {
    code: String,
}

/// POST /pair 用配对码换取 bearer token
pub async fn pair_with_code(port: u16, code: &str) -> Result<String> {
    let url = format!("{}/pair", base_url(port));
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let res = client
        .post(&url)
        .header("X-Pairing-Code", code)
        .send()
        .await?;
    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("pair 失败: {}", text);
    }
    let body: PairResponse = res.json().await.context("解析 pair 响应失败")?;
    body.token
        .ok_or_else(|| anyhow::anyhow!("响应中无 token"))
}

#[derive(Deserialize)]
struct PairResponse {
    token: Option<String>,
}

/// 桌面端 token 存储路径（与 zeroclaw 目录一致）
pub fn desktop_token_path() -> Result<std::path::PathBuf> {
    let dir = dirs::config_dir().context("无法获取 config 目录")?;
    let zeroclaw = dir.join("zeroclaw");
    std::fs::create_dir_all(&zeroclaw)?;
    Ok(zeroclaw.join("desktop-token"))
}

/// 读取已保存的 token
pub fn load_saved_token() -> Result<Option<String>> {
    let p = desktop_token_path()?;
    if p.exists() {
        let s = std::fs::read_to_string(&p)?;
        let s = s.trim().to_string();
        if !s.is_empty() {
            return Ok(Some(s));
        }
    }
    Ok(None)
}

/// 保存 token 到文件
pub fn save_token(token: &str) -> Result<()> {
    let p = desktop_token_path()?;
    std::fs::write(p, token)?;
    Ok(())
}

/// 带 Bearer 的 GET /api/status
pub async fn api_status(state: &GatewayState) -> Result<serde_json::Value> {
    let url = format!("{}/api/status", state.base_url());
    let mut req = Client::new().get(&url);
    if let Some(h) = state.auth_header() {
        req = req.header("Authorization", h);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        let t = res.text().await.unwrap_or_default();
        anyhow::bail!("GET /api/status 失败: {}", t);
    }
    let json = res.json().await?;
    Ok(json)
}

/// 带 Bearer 的 GET /api/config（返回 { format, content }，content 为 TOML 字符串）
pub async fn api_config_get(state: &GatewayState) -> Result<ApiConfigResponse> {
    let url = format!("{}/api/config", state.base_url());
    let mut req = Client::new().get(&url);
    if let Some(h) = state.auth_header() {
        req = req.header("Authorization", h);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        let t = res.text().await.unwrap_or_default();
        anyhow::bail!("GET /api/config 失败: {}", t);
    }
    let json: ApiConfigResponse = res.json().await?;
    Ok(json)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfigResponse {
    pub format: String,
    pub content: String,
}

/// 带 Bearer 的 PUT /api/config（body 为 TOML 字符串）
pub async fn api_config_put(state: &GatewayState, toml_body: &str) -> Result<()> {
    let url = format!("{}/api/config", state.base_url());
    let mut req = Client::new()
        .put(&url)
        .header("Content-Type", "application/toml")
        .body(toml_body.to_string());
    if let Some(h) = state.auth_header() {
        req = req.header("Authorization", h);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        let t = res.text().await.unwrap_or_default();
        anyhow::bail!("PUT /api/config 失败: {}", t);
    }
    Ok(())
}

/// 带 Bearer 的 GET /api/tools
pub async fn api_tools(state: &GatewayState) -> Result<serde_json::Value> {
    let url = format!("{}/api/tools", state.base_url());
    let mut req = Client::new().get(&url);
    if let Some(h) = state.auth_header() {
        req = req.header("Authorization", h);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        let t = res.text().await.unwrap_or_default();
        anyhow::bail!("GET /api/tools 失败: {}", t);
    }
    let json = res.json().await?;
    Ok(json)
}
