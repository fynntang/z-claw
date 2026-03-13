use crate::agent::AgentResponse;
use zeroclaw::providers::create_provider_with_url;
use zeroclaw::providers::ChatMessage as ProviderChatMessage;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use super::{ChatInput, RuntimeStatus};

/// Transitional desktop bridge that isolates Tauri command handlers
/// from runtime/provider implementation details.
pub struct DesktopBridge;

static SESSION_HISTORY: LazyLock<RwLock<HashMap<String, Vec<ProviderChatMessage>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static SESSION_COUNTER: AtomicU64 = AtomicU64::new(1);

fn generate_session_id() -> String {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64);
    let seq = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("sess-{now_ms}-{seq}")
}

impl DesktopBridge {
    pub async fn chat(input: ChatInput) -> AgentResponse {
        let provider_name = input.provider.as_deref().unwrap_or("openai");
        let model = input.model.unwrap_or_else(|| "gpt-4o-mini".to_string());
        let api_key = input
            .api_key
            .filter(|key| !key.trim().is_empty());
        let session_id = input.session_id.unwrap_or_else(generate_session_id);

        let provider = create_provider_with_url(
            provider_name,
            api_key.as_deref(),
            input.api_url.as_deref(),
        );

        match provider {
            Ok(provider) => {
                let mut history = {
                    let sessions = SESSION_HISTORY.read().await;
                    sessions.get(&session_id).cloned().unwrap_or_default()
                };
                history.push(ProviderChatMessage::user(input.message));

                match provider.chat_with_history(&history, &model, 0.7).await {
                    Ok(content) => {
                        history.push(ProviderChatMessage::assistant(content.clone()));
                        {
                            let mut sessions = SESSION_HISTORY.write().await;
                            sessions.insert(session_id.clone(), history);
                        }

                        AgentResponse {
                            content,
                            success: true,
                            error: None,
                            session_id: Some(session_id),
                        }
                    }
                    Err(e) => AgentResponse {
                        content: String::new(),
                        success: false,
                        error: Some(e.to_string()),
                        session_id: Some(session_id),
                    },
                }
            }
            Err(e) => AgentResponse {
                content: String::new(),
                success: false,
                error: Some(e.to_string()),
                session_id: Some(session_id),
            },
        }
    }

    pub fn status() -> RuntimeStatus {
        RuntimeStatus {
            running: false,
            provider: "none".to_string(),
            model: "none".to_string(),
        }
    }
}
