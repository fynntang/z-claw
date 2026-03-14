//! Configuration bridge for Z-Claw desktop.
//!
//! Uses local config/workspace paths (~/.zeroclaw) and zeroclaw::Config only for parsing/defaults.
//! Does not modify zeroclaw; autonomy 等通过 TOML 读写，不依赖 zeroclaw::AutonomyLevel。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Value;

/// GUI-facing configuration structure (subset of zeroclaw::Config used by settings UI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // ── 提供商 / 模型 ──
    pub api_key: String,
    pub api_url: String,
    pub provider: String,
    pub model: String,
    pub temperature: f64,
    pub local_model_path: Option<String>,

    // ── 请求与超时 ──
    /// HTTP 超时秒数，默认 120
    pub provider_timeout_secs: u64,
    /// 自定义 API 路径后缀，如 /v2/generate
    pub api_path: Option<String>,

    // ── Agent [agent] ──
    pub max_tool_iterations: u32,
    pub max_history_messages: u32,
    pub compact_context: bool,
    pub parallel_tools: bool,

    // ── Memory [memory] ──
    pub memory_backend: String,
    pub memory_auto_save: bool,

    // ── 自主性 [autonomy] ──
    /// "readonly" | "supervised" | "full"
    pub autonomy_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let c = zeroclaw::Config::default();
        Self {
            api_key: c.api_key.unwrap_or_default(),
            api_url: c
                .api_url
                .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string()),
            provider: c
                .default_provider
                .unwrap_or_else(|| "openrouter".to_string()),
            model: c
                .default_model
                .unwrap_or_else(|| "anthropic/claude-sonnet-4.6".to_string()),
            temperature: c.default_temperature,
            local_model_path: None,
            provider_timeout_secs: c.provider_timeout_secs,
            api_path: c.api_path.clone(),
            max_tool_iterations: c.agent.max_tool_iterations as u32,
            max_history_messages: c.agent.max_history_messages as u32,
            compact_context: c.agent.compact_context,
            parallel_tools: c.agent.parallel_tools,
            memory_backend: c.memory.backend.clone(),
            memory_auto_save: c.memory.auto_save,
            autonomy_level: "supervised".into(),
        }
    }
}

/// 返回 (config_dir, workspace_dir)，与 zeroclaw 约定一致：~/.zeroclaw 与 ~/.zeroclaw/workspace。
/// 不依赖 zeroclaw 内部 API，便于在未修改 zeroclaw 时使用。
fn get_zeroclaw_dirs_local() -> (PathBuf, PathBuf) {
    let config_dir = dirs::home_dir()
        .map(|h| h.join(".zeroclaw"))
        .unwrap_or_else(|| PathBuf::from(".").join("zeroclaw"));
    let workspace_dir = config_dir.join("workspace");
    (config_dir, workspace_dir)
}

/// Configuration validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}


/// Get the config file path (~/.zeroclaw/config.toml by default).
pub fn get_config_path() -> PathBuf {
    get_zeroclaw_dirs_local().0.join("config.toml")
}

/// Read configuration from ZeroClaw config file.
/// Returns default config if file doesn't exist.
pub fn get_config() -> Result<AppConfig, String> {
    let config_path = get_config_path();

    if !config_path.exists() {
        // Return default config if file doesn't exist
        return Ok(AppConfig::default());
    }

    // Read config file
    let contents = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    // 从 TOML 读取 autonomy.level（不依赖 zeroclaw::AutonomyLevel）
    let autonomy_level: String = toml::from_str::<Value>(&contents)
        .ok()
        .and_then(|v| v.get("autonomy").and_then(|a| a.get("level")).and_then(|l| l.as_str().map(String::from)))
        .unwrap_or_else(|| "supervised".into());

    let config: zeroclaw::Config =
        toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))?;

    let default_c = zeroclaw::Config::default();
    let app_config = AppConfig {
        api_key: config.api_key.unwrap_or_default(),
        api_url: config.api_url.unwrap_or_else(|| {
            default_c
                .api_url
                .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string())
        }),
        provider: config
            .default_provider
            .unwrap_or_else(|| default_c.default_provider.unwrap_or_else(|| "openrouter".to_string())),
        model: config
            .default_model
            .unwrap_or_else(|| default_c.default_model.unwrap_or_else(|| "anthropic/claude-sonnet-4.6".to_string())),
        temperature: config.default_temperature,
        local_model_path: None,
        provider_timeout_secs: config.provider_timeout_secs,
        api_path: config.api_path.clone(),
        max_tool_iterations: config.agent.max_tool_iterations as u32,
        max_history_messages: config.agent.max_history_messages as u32,
        compact_context: config.agent.compact_context,
        parallel_tools: config.agent.parallel_tools,
        memory_backend: config.memory.backend.clone(),
        memory_auto_save: config.memory.auto_save,
        autonomy_level,
    };

    Ok(app_config)
}

/// Save configuration to ZeroClaw config file.
/// Preserves non-GUI fields from existing config.
pub fn set_config(config: AppConfig) -> Result<(), String> {
    // Validate first
    let validation = validate_config(&config);
    if !validation.valid {
        return Err(format!(
            "Validation failed: {}",
            validation.errors.join(", ")
        ));
    }

    let (config_dir, _) = get_zeroclaw_dirs_local();
    let config_path = get_config_path();

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 以 TOML Value 读写，避免依赖 zeroclaw 内部类型（如 AutonomyLevel）
    let mut value: Value = if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read existing config: {}", e))?;
        toml::from_str(&contents).unwrap_or_else(|_| Value::Table(toml::map::Map::new()))
    } else {
        Value::Table(toml::map::Map::new())
    };

    let table = value.as_table_mut().ok_or("Config is not a TOML table")?;

    // 顶层字段
    table.insert("api_key".into(), Value::String(config.api_key));
    table.insert("api_url".into(), Value::String(config.api_url));
    table.insert("default_provider".into(), Value::String(config.provider));
    table.insert("default_model".into(), Value::String(config.model));
    table.insert("default_temperature".into(), Value::Float(config.temperature));
    table.insert("provider_timeout_secs".into(), Value::Integer(config.provider_timeout_secs as i64));
    if let Some(p) = &config.api_path {
        table.insert("api_path".into(), Value::String(p.clone()));
    } else {
        table.remove("api_path");
    }

    // [agent]
    let agent = table.entry("agent").or_insert_with(|| Value::Table(toml::map::Map::new()));
    if let Some(t) = agent.as_table_mut() {
        t.insert("max_tool_iterations".into(), Value::Integer(config.max_tool_iterations as i64));
        t.insert("max_history_messages".into(), Value::Integer(config.max_history_messages as i64));
        t.insert("compact_context".into(), Value::Boolean(config.compact_context));
        t.insert("parallel_tools".into(), Value::Boolean(config.parallel_tools));
    }

    // [memory]
    let memory = table.entry("memory").or_insert_with(|| Value::Table(toml::map::Map::new()));
    if let Some(t) = memory.as_table_mut() {
        t.insert("backend".into(), Value::String(config.memory_backend));
        t.insert("auto_save".into(), Value::Boolean(config.memory_auto_save));
    }

    // [autonomy] level：zeroclaw 使用小写 "readonly" | "supervised" | "full"
    let autonomy_str = match config.autonomy_level.trim().to_lowercase().as_str() {
        "read_only" | "readonly" => "readonly",
        "full" => "full",
        _ => "supervised",
    };
    let autonomy = table.entry("autonomy").or_insert_with(|| Value::Table(toml::map::Map::new()));
    if let Some(t) = autonomy.as_table_mut() {
        t.insert("level".into(), Value::String(autonomy_str.into()));
    }

    let toml_contents = toml::to_string_pretty(&value)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    // Write to file
    std::fs::write(&config_path, toml_contents)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // Set file permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("Failed to set file permissions: {}", e))?;
    }

    Ok(())
}

/// Validate configuration.
pub fn validate_config(config: &AppConfig) -> ValidationResult {
    let mut errors = Vec::new();

    // API key should not be empty for cloud providers
    if config.api_key.trim().is_empty()
        && !config.provider.starts_with("local")
        && config.provider != "ollama"
    {
        // Only warn, don't error - user might set it later
    }

    // Validate API URL format
    if !config.api_url.is_empty() {
        if !config.api_url.starts_with("http://") && !config.api_url.starts_with("https://") {
            errors.push("API URL must start with http:// or https://".to_string());
        }
    }

    // Validate model name
    if config.model.trim().is_empty() {
        errors.push("Model name cannot be empty".to_string());
    }

    if config.temperature < 0.0 || config.temperature > 2.0 {
        errors.push("Temperature must be between 0.0 and 2.0".to_string());
    }

    if config.provider_timeout_secs == 0 {
        errors.push("Provider timeout must be greater than 0".to_string());
    }
    if config.max_tool_iterations == 0 {
        errors.push("Max tool iterations must be greater than 0".to_string());
    }
    if config.max_history_messages == 0 {
        errors.push("Max history messages must be greater than 0".to_string());
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(config.api_key.is_empty());
        assert!(!config.api_url.is_empty());
        assert_eq!(config.provider, "openrouter");
        assert!(!config.model.is_empty());
    }

    #[test]
    fn test_validate_config_valid() {
        let config = AppConfig {
            api_key: "sk-test".to_string(),
            api_url: "https://openrouter.ai/api/v1".to_string(),
            provider: "openrouter".to_string(),
            model: "anthropic/claude-sonnet-4.6".to_string(),
            temperature: 0.7,
            ..AppConfig::default()
        };
        let result = validate_config(&config);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_config_invalid_temperature() {
        let config = AppConfig {
            temperature: 3.0,
            ..AppConfig::default()
        };
        let result = validate_config(&config);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Temperature")));
    }

    #[test]
    fn test_validate_config_invalid_url() {
        let config = AppConfig {
            api_url: "invalid-url".to_string(),
            ..AppConfig::default()
        };
        let result = validate_config(&config);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("API URL")));
    }
}
