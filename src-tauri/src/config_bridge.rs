//! 将 zeroclaw Gateway 返回的 TOML 配置与前端 ConfigState 互转。

use serde::{Deserialize, Serialize};

/// 前端使用的配置状态（与 Settings.svelte ConfigState 一致）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigState {
    pub api_key: String,
    pub api_url: String,
    pub provider: String,
    pub model: String,
    pub temperature: f64,
    pub local_model_path: Option<String>,
    pub provider_timeout_secs: u64,
    pub api_path: Option<String>,
    pub max_tool_iterations: u32,
    pub max_history_messages: u32,
    pub compact_context: bool,
    pub parallel_tools: bool,
    pub memory_backend: String,
    pub memory_auto_save: bool,
    pub autonomy_level: String,
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_url: "https://openrouter.ai/api/v1".into(),
            provider: "openrouter".into(),
            model: "anthropic/claude-sonnet-4.6".into(),
            temperature: 0.7,
            local_model_path: None,
            provider_timeout_secs: 120,
            api_path: None,
            max_tool_iterations: 10,
            max_history_messages: 50,
            compact_context: false,
            parallel_tools: false,
            memory_backend: "sqlite".into(),
            memory_auto_save: true,
            autonomy_level: "supervised".into(),
        }
    }
}

/// 从 Gateway 返回的 TOML 字符串解析为 ConfigState
pub fn toml_to_config_state(toml_str: &str) -> Result<ConfigState, toml::de::Error> {
    let value: toml::Value = toml::from_str(toml_str)?;
    let mut c = ConfigState::default();
    if let Some(api_key) = value.get("api_key").and_then(|v| v.as_str()) {
        c.api_key = api_key.to_string();
    }
    if let Some(url) = value.get("api_url").and_then(|v| v.as_str()) {
        c.api_url = url.to_string();
    }
    if let Some(p) = value.get("default_provider").and_then(|v| v.as_str()) {
        c.provider = p.to_string();
    }
    if let Some(m) = value.get("default_model").and_then(|v| v.as_str()) {
        c.model = m.to_string();
    }
    if let Some(agent) = value.get("agent").and_then(|v| v.as_table()) {
        if let Some(t) = agent.get("temperature").and_then(|v| v.as_float()) {
            c.temperature = t;
        }
        if let Some(n) = agent.get("max_tool_iterations").and_then(|v| v.as_integer()) {
            c.max_tool_iterations = n as u32;
        }
        if let Some(n) = agent.get("max_history_messages").and_then(|v| v.as_integer()) {
            c.max_history_messages = n as u32;
        }
        if let Some(b) = agent.get("compact_context").and_then(|v| v.as_bool()) {
            c.compact_context = b;
        }
        if let Some(b) = agent.get("parallel_tools").and_then(|v| v.as_bool()) {
            c.parallel_tools = b;
        }
    }
    if let Some(timeout) = value.get("provider_timeout_secs").and_then(|v| v.as_integer()) {
        c.provider_timeout_secs = timeout as u64;
    }
    if let Some(path) = value.get("api_path").and_then(|v| v.as_str()) {
        c.api_path = Some(path.to_string());
    }
    if let Some(mem) = value.get("memory").and_then(|v| v.as_table()) {
        if let Some(b) = mem.get("backend").and_then(|v| v.as_str()) {
            c.memory_backend = b.to_string();
        }
        if let Some(b) = mem.get("auto_save").and_then(|v| v.as_bool()) {
            c.memory_auto_save = b;
        }
    }
    if let Some(autonomy) = value.get("autonomy").and_then(|v| v.as_table()) {
        if let Some(l) = autonomy.get("level").and_then(|v| v.as_str()) {
            c.autonomy_level = l.to_string();
        }
    }
    Ok(c)
}

/// 将当前 TOML 字符串与 ConfigState 合并，生成新的 TOML 字符串（用于 PUT）
pub fn merge_config_state_into_toml(current_toml: &str, state: &ConfigState) -> Result<String, String> {
    let mut value: toml::Value = toml::from_str(current_toml).map_err(|e| e.to_string())?;
    if let Some(t) = value.as_table_mut() {
        t.insert("api_key".into(), toml::Value::String(state.api_key.clone()));
        t.insert("api_url".into(), toml::Value::String(state.api_url.clone()));
        t.insert("default_provider".into(), toml::Value::String(state.provider.clone()));
        t.insert("default_model".into(), toml::Value::String(state.model.clone()));
        t.insert("provider_timeout_secs".into(), toml::Value::Integer(state.provider_timeout_secs as i64));
        if let Some(path) = &state.api_path {
            t.insert("api_path".into(), toml::Value::String(path.clone()));
        }
        if !t.contains_key("agent") {
            t.insert("agent".into(), toml::Value::Table(toml::map::Map::new()));
        }
        if let Some(agent) = t.get_mut("agent").and_then(|v| v.as_table_mut()) {
            agent.insert("temperature".into(), toml::Value::Float(state.temperature));
            agent.insert("max_tool_iterations".into(), toml::Value::Integer(state.max_tool_iterations as i64));
            agent.insert("max_history_messages".into(), toml::Value::Integer(state.max_history_messages as i64));
            agent.insert("compact_context".into(), toml::Value::Boolean(state.compact_context));
            agent.insert("parallel_tools".into(), toml::Value::Boolean(state.parallel_tools));
        }
        if !t.contains_key("memory") {
            t.insert("memory".into(), toml::Value::Table(toml::map::Map::new()));
        }
        if let Some(mem) = t.get_mut("memory").and_then(|v| v.as_table_mut()) {
            mem.insert("backend".into(), toml::Value::String(state.memory_backend.clone()));
            mem.insert("auto_save".into(), toml::Value::Boolean(state.memory_auto_save));
        }
        if !t.contains_key("autonomy") {
            t.insert("autonomy".into(), toml::Value::Table(toml::map::Map::new()));
        }
        if let Some(autonomy) = t.get_mut("autonomy").and_then(|v| v.as_table_mut()) {
            autonomy.insert("level".into(), toml::Value::String(state.autonomy_level.clone()));
        }
    }
    toml::to_string_pretty(&value).map_err(|e| e.to_string())
}
