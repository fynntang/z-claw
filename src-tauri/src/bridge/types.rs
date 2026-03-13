use serde::{Deserialize, Serialize};

/// UI-facing input contract for a single chat turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInput {
    pub session_id: Option<String>,
    pub message: String,
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
}

/// Runtime status snapshot for desktop UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub running: bool,
    pub provider: String,
    pub model: String,
}
