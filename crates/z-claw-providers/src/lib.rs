mod anthropic;
mod openai;
mod routing;

pub use anthropic::AnthropicProvider;
pub use openai::OpenAiProvider;
pub use routing::ProviderChain;

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use z_claw_core::{ChatMessage, Feature, GenerateConfig, ModelInfo, StreamChunk, ToolDef};

pub type ChunkStream =
    Pin<Box<dyn Stream<Item = Result<StreamChunk, z_claw_core::ClawError>> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<ToolDef>,
        config: &GenerateConfig,
    ) -> Result<ChunkStream, z_claw_core::ClawError>;

    async fn chat_complete(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<ToolDef>,
        config: &GenerateConfig,
    ) -> Result<String, z_claw_core::ClawError> {
        let mut stream = self.chat(messages, tools, config).await?;
        let mut text = String::new();
        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            match chunk? {
                StreamChunk::TextDelta(delta) => text.push_str(&delta),
                StreamChunk::Done { .. } => break,
                _ => {}
            }
        }
        Ok(text)
    }

    fn list_models(&self) -> Vec<ModelInfo>;
    fn supports_feature(&self, feature: Feature) -> bool;
}
