use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use z_claw_core::Platform;

// --- Config schema ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_provider_id")]
    pub default_provider_id: String,

    #[serde(default = "default_model")]
    pub default_model: String,

    #[serde(default)]
    pub providers: Vec<ProviderDef>,

    #[serde(default)]
    pub routing: RoutingConfig,

    #[serde(default)]
    pub mcp_servers: Vec<McpServerDef>,

    #[serde(default)]
    pub policy: PolicyConfig,

    #[serde(default)]
    pub memory: MemoryConfig,

    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDef {
    pub id: String,
    pub base_url: String,
    #[serde(default)]
    pub api_key_env: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    #[serde(default)]
    pub fallback_chain: Vec<String>,
    #[serde(default)]
    pub complexity_rules: ComplexityRules,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplexityRules {
    pub simple: Option<String>,
    pub medium: Option<String>,
    pub complex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDef {
    pub id: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub lazy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default = "default_require_approval")]
    pub require_tool_approval: bool,
    #[serde(default)]
    pub blocked_commands: Vec<String>,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default = "default_security_level")]
    pub default_security_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(default = "default_compaction_threshold")]
    pub compaction_threshold: usize,
    #[serde(default = "default_compaction_keep")]
    pub compaction_keep_recent: usize,
    #[serde(default = "default_true")]
    pub compaction_enabled: bool,
    #[serde(default)]
    pub embedding_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default)]
    pub theme: String,
}

// --- Default functions ---

fn default_provider_id() -> String { "ollama".into() }
fn default_model() -> String { "llama3".into() }
fn default_require_approval() -> bool { true }
fn default_security_level() -> String { "confirm_execute".into() }
fn default_compaction_threshold() -> usize { 50 }
fn default_compaction_keep() -> usize { 10 }
fn default_true() -> bool { true }
fn default_font_size() -> f32 { 14.0 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_provider_id: default_provider_id(),
            default_model: default_model(),
            providers: vec![
                ProviderDef {
                    id: "ollama".into(),
                    base_url: "http://localhost:11434/v1".into(),
                    api_key_env: None,
                    api_key: Some("ollama".into()),
                    default_model: Some("llama3".into()),
                },
            ],
            routing: RoutingConfig::default(),
            mcp_servers: vec![],
            policy: PolicyConfig::default(),
            memory: MemoryConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self { fallback_chain: vec![], complexity_rules: ComplexityRules::default() }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            require_tool_approval: default_require_approval(),
            blocked_commands: vec![],
            allowed_paths: vec!["~".into()],
            default_security_level: default_security_level(),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            compaction_threshold: default_compaction_threshold(),
            compaction_keep_recent: default_compaction_keep(),
            compaction_enabled: default_true(),
            embedding_model: None,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self { font_size: default_font_size(), theme: "dark".into() }
    }
}

// --- Config loading ---

/// Load config from the standard location, or return defaults.
pub fn load_config(platform: &dyn Platform) -> AppConfig {
    let path = config_path(platform);
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            toml::from_str(&content).unwrap_or_else(|e| {
                tracing::warn!("Failed to parse config at {:?}: {e}, using defaults", path);
                AppConfig::default()
            })
        }
        Err(_) => {
            tracing::info!("No config file at {:?}, using defaults", path);
            let config = AppConfig::default();
            // Write default config so user can edit it
            if let Ok(toml_str) = toml::to_string_pretty(&config) {
                let _ = std::fs::create_dir_all(path.parent().unwrap());
                let _ = std::fs::write(&path, toml_str);
            }
            config
        }
    }
}

pub fn config_path(platform: &dyn Platform) -> PathBuf {
    platform.config_dir().join("config.toml")
}

// --- API key resolution ---

pub fn resolve_api_key(def: &ProviderDef) -> Option<String> {
    // 1. Check environment variable
    if let Some(env_var) = &def.api_key_env {
        if let Ok(val) = std::env::var(env_var) {
            if !val.trim().is_empty() {
                return Some(val);
            }
        }
    }
    // 2. Fall back to inline key
    def.api_key.clone().filter(|k| !k.trim().is_empty())
}

mod keybindings;
pub use keybindings::{KeyBinding, KeybindingConfig};
