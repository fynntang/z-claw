//! Agent module - LLM API integration
//! 
//! Provides a simple OpenAI-compatible API client for AI conversations.

use anyhow::{Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// API key for the LLM provider
    pub api_key: Option<String>,
    /// API base URL (default: https://api.openai.com/v1)
    pub api_url: Option<String>,
    /// Provider to use
    pub provider: Option<String>,
    /// Model to use (default: gpt-4o-mini)
    pub model: Option<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_url: Some("https://api.openai.com/v1".to_string()),
            provider: Some("openai".to_string()),
            model: Some("gpt-4o-mini".to_string()),
        }
    }
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// OpenAI chat request
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

/// OpenAI chat response
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// Agent response for Tauri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Send a chat message to the LLM
pub async fn send_message(message: &str, config: AgentConfig) -> Result<AgentResponse> {
    let api_key = match config.api_key {
        Some(key) if !key.is_empty() => key,
        _ => bail!("API key not configured"),
    };
    
    let api_url = config.api_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    let model = config.model.unwrap_or_else(|| "gpt-4o-mini".to_string());
    
    let client = Client::new();
    
    let request = ChatRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        }],
        stream: false,
    };
    
    let response = client
        .post(format!("{}/chat/completions", api_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        bail!("API error: {}", error_text);
    }
    
    let chat_response: ChatResponse = response.json().await?;
    
    let content = chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "No response".to_string());
    
    Ok(AgentResponse {
        content,
        success: true,
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert!(config.api_key.is_none());
        assert!(config.model.is_some());
    }
}