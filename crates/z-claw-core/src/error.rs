use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ClawError {
    #[error("config: {0}")]
    Config(String),

    #[error("provider [{provider}]: {message}")]
    Provider { provider: String, message: String },

    #[error("tool [{tool}]: {message}")]
    Tool { tool: String, message: String },

    #[error("memory: {0}")]
    Memory(String),

    #[error("security: {0}")]
    Security(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("http error: {0}")]
    Http(String),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(String),

    #[error("all providers failed")]
    AllProvidersFailed,

    #[error("agent loop exceeded max rounds ({0})")]
    MaxRoundsExceeded(usize),

    #[error("approval timed out for tool [{tool}]")]
    ApprovalTimeout { tool: String },

    #[error("tool blocked: {tool} — {reason}")]
    ToolBlocked { tool: String, reason: String },

    #[error("path not allowed: {0}")]
    PathNotAllowed(PathBuf),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ClawError>;
