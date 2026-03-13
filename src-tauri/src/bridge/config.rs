//! Configuration bridge for Z-Claw desktop.
//!
//! Provides secure configuration management using ZeroClaw's config system.
//! API keys are encrypted at rest and never stored in localStorage.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// GUI-facing configuration structure.
/// This is the contract between frontend and backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// API key (decrypted when returned to frontend)
    pub api_key: String,
    /// Custom API endpoint URL
    pub api_url: String,
    /// Provider name (e.g., "openai", "anthropic", "openrouter")
    pub provider: String,
    /// Default model name
    pub model: String,
    /// Temperature parameter (0.0 - 2.0)
    pub temperature: f64,
    /// Local model path (optional, for local inference)
    pub local_model_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_url: "https://api.openai.com/v1".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.7,
            local_model_path: None,
        }
    }
}

/// Configuration validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Get the ZeroClaw config directory path.
fn get_zeroclaw_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zeroclaw")
}

/// Get the config file path.
pub fn get_config_path() -> PathBuf {
    get_zeroclaw_config_dir().join("config.toml")
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

    // Extract GUI-relevant fields
    let app_config = AppConfig {
        api_key: config.api_key.unwrap_or_default(),
        api_url: config
            .api_url
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
        provider: config
            .default_provider
            .unwrap_or_else(|| "openai".to_string()),
        model: config
            .default_model
            .unwrap_or_else(|| "gpt-4o-mini".to_string()),
        temperature: config.default_temperature,
        local_model_path: None, // ZeroClaw doesn't have this field yet
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

    let config_dir = get_zeroclaw_config_dir();
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

    // Enable encryption for secrets by default
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

    // Validate temperature range
    if config.temperature < 0.0 || config.temperature > 2.0 {
        errors.push("Temperature must be between 0.0 and 2.0".to_string());
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
        assert!(!config.model.is_empty());
    }

    #[test]
    fn test_validate_config_valid() {
        let config = AppConfig {
            api_key: "sk-test".to_string(),
            api_url: "https://api.openai.com/v1".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.7,
            local_model_path: None,
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
