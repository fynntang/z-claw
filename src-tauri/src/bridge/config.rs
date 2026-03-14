//! Configuration bridge for Z-Claw desktop.
//!
//! Uses zeroclaw::Config and default_config_and_workspace_dirs() for paths.
//! GUI fields map to Config top-level: api_key, api_url, default_provider, default_model, default_temperature.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
            autonomy_level: autonomy_level_to_string(&c.autonomy.level),
        }
    }
}

fn autonomy_level_to_string(level: &zeroclaw::AutonomyLevel) -> String {
    match level {
        zeroclaw::AutonomyLevel::ReadOnly => "readonly".into(),
        zeroclaw::AutonomyLevel::Supervised => "supervised".into(),
        zeroclaw::AutonomyLevel::Full => "full".into(),
    }
}

fn parse_autonomy_level(s: &str) -> zeroclaw::AutonomyLevel {
    match s.trim().to_lowercase().as_str() {
        "read_only" | "readonly" => zeroclaw::AutonomyLevel::ReadOnly,
        "full" => zeroclaw::AutonomyLevel::Full,
        _ => zeroclaw::AutonomyLevel::Supervised,
    }
}

/// Configuration validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Get (config_dir, workspace_dir) from zeroclaw default_config_and_workspace_dirs().
fn get_zeroclaw_dirs() -> (PathBuf, PathBuf) {
    zeroclaw::config::schema::default_config_and_workspace_dirs()
        .unwrap_or_else(|_| (PathBuf::from(".").join("zeroclaw"), PathBuf::from(".").join("zeroclaw").join("workspace")))
}

/// Get the config file path (~/.zeroclaw/config.toml by default).
pub fn get_config_path() -> PathBuf {
    get_zeroclaw_dirs().0.join("config.toml")
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

    // Parse TOML
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
        autonomy_level: autonomy_level_to_string(&config.autonomy.level),
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

    let (config_dir, _) = get_zeroclaw_dirs();
    let config_path = get_config_path();

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // Read existing config or create default
    let mut zeroclaw_config = if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read existing config: {}", e))?;
        toml::from_str(&contents).unwrap_or_else(|_| zeroclaw::Config::default())
    } else {
        zeroclaw::Config::default()
    };

    // Update GUI-relevant fields
    zeroclaw_config.api_key = Some(config.api_key);
    zeroclaw_config.api_url = Some(config.api_url);
    zeroclaw_config.default_provider = Some(config.provider);
    zeroclaw_config.default_model = Some(config.model);
    zeroclaw_config.default_temperature = config.temperature;
    zeroclaw_config.provider_timeout_secs = config.provider_timeout_secs;
    zeroclaw_config.api_path = config.api_path.clone();

    zeroclaw_config.agent.max_tool_iterations = config.max_tool_iterations as usize;
    zeroclaw_config.agent.max_history_messages = config.max_history_messages as usize;
    zeroclaw_config.agent.compact_context = config.compact_context;
    zeroclaw_config.agent.parallel_tools = config.parallel_tools;

    zeroclaw_config.memory.backend = config.memory_backend.clone();
    zeroclaw_config.memory.auto_save = config.memory_auto_save;

    zeroclaw_config.autonomy.level = parse_autonomy_level(&config.autonomy_level);

    zeroclaw_config.secrets.encrypt = true;

    // Serialize to TOML
    let toml_contents = toml::to_string_pretty(&zeroclaw_config)
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
