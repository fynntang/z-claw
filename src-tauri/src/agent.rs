//! Transitional agent-facing DTOs.
//!
//! Runtime execution now routes through `bridge` + `zeroclaw` integration.

use serde::{Deserialize, Serialize};

/// Chat command response returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
    pub session_id: Option<String>,
}
