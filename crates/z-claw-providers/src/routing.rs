use super::*;
use std::sync::Arc;

pub struct ProviderChain {
    providers: Vec<Arc<dyn LlmProvider>>,
}

impl ProviderChain {
    pub fn new(providers: Vec<Arc<dyn LlmProvider>>) -> Self {
        Self { providers }
    }

    pub fn from_single(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            providers: vec![provider],
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<ToolDef>,
        config: &GenerateConfig,
    ) -> Result<ChunkStream, z_claw_core::ClawError> {
        let mut last_error = None;
        for provider in &self.providers {
            match provider.chat(messages.clone(), tools.clone(), config).await {
                Ok(stream) => return Ok(stream),
                Err(e) => {
                    tracing::warn!("Provider fallback: {}", e);
                    last_error = Some(e);
                }
            }
        }
        Err(last_error.unwrap_or(z_claw_core::ClawError::AllProvidersFailed))
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}
