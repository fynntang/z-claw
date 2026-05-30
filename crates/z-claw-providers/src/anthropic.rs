use super::*;
use futures::StreamExt;
use reqwest::Client;

pub struct AnthropicProvider {
    pub id: String,
    pub api_key: String,
    pub default_model: String,
    client: Client,
}

impl AnthropicProvider {
    pub fn new(id: String, api_key: String, default_model: String) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        Self {
            id,
            api_key,
            default_model,
            client,
        }
    }

    fn endpoint() -> &'static str {
        "https://api.anthropic.com/v1/messages"
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<ToolDef>,
        config: &GenerateConfig,
    ) -> Result<ChunkStream, z_claw_core::ClawError> {
        let model = if config.model.is_empty() {
            &self.default_model
        } else {
            &config.model
        };

        let system = messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());

        let anthropic_msgs: Vec<serde_json::Value> = messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| {
                let mut msg = serde_json::json!({"role": m.role, "content": m.content});
                // Tool result messages need special Anthropic format
                if m.role == "tool" {
                    if let Some(tc_id) = &m.tool_call_id {
                        msg = serde_json::json!({
                            "role": "user",
                            "content": [{
                                "type": "tool_result",
                                "tool_use_id": tc_id,
                                "content": m.content,
                            }]
                        });
                    }
                }
                msg
            })
            .collect();

        let tool_specs: Vec<serde_json::Value> = tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters,
                })
            })
            .collect();

        let mut body = serde_json::json!({
            "model": model,
            "max_tokens": config.max_tokens.unwrap_or(4096),
            "stream": true,
            "messages": anthropic_msgs,
        });
        if let Some(sys) = system {
            body["system"] = serde_json::json!(sys);
        }
        if !tool_specs.is_empty() {
            body["tools"] = serde_json::json!(tool_specs);
        }
        if let Some(temp) = config.temperature {
            body["temperature"] = serde_json::json!(temp);
        }

        let resp = self
            .client
            .post(Self::endpoint())
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("anthropic-beta", "tools-2024-04-04")
            .json(&body)
            .send()
            .await
            .map_err(|e| z_claw_core::ClawError::Provider {
                provider: self.id.clone(),
                message: e.to_string(),
            })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(z_claw_core::ClawError::Provider {
                provider: self.id.clone(),
                message: format!("HTTP {status}: {text}"),
            });
        }

        let byte_stream = resp.bytes_stream();
        let stream = byte_stream.map(|result: reqwest::Result<_>| {
            let bytes = result.map_err(|e| z_claw_core::ClawError::Http(e.to_string()))?;
            let text = String::from_utf8_lossy(&bytes);
            parse_anthropic_sse(&text)
        });

        Ok(Box::pin(stream))
    }

    fn list_models(&self) -> Vec<ModelInfo> {
        vec![ModelInfo {
            id: self.default_model.clone(),
            display_name: self.default_model.clone(),
            provider_id: self.id.clone(),
        }]
    }

    fn supports_feature(&self, feature: Feature) -> bool {
        matches!(
            feature,
            Feature::ToolCalling | Feature::Streaming | Feature::Vision | Feature::Reasoning
        )
    }
}

fn parse_anthropic_sse(text: &str) -> Result<StreamChunk, z_claw_core::ClawError> {
    let mut event_type = String::new();
    let mut data = String::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("event: ") {
            event_type = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("data: ") {
            data = rest.to_string();
        }
    }

    if data.is_empty() {
        return Ok(StreamChunk::TextDelta(String::new()));
    }

    match event_type.as_str() {
        "message_stop" => Ok(StreamChunk::Done {
            finish_reason: Some("end_turn".into()),
        }),
        "content_block_start" => {
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            let block = &parsed["content_block"];
            if block["type"].as_str() == Some("tool_use") {
                Ok(StreamChunk::ToolCallStart {
                    index: parsed["index"].as_u64().unwrap_or(0) as usize,
                    id: block["id"].as_str().unwrap_or("").to_string(),
                    name: block["name"].as_str().unwrap_or("").to_string(),
                })
            } else {
                Ok(StreamChunk::TextDelta(String::new()))
            }
        }
        "content_block_delta" => {
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            let delta = &parsed["delta"];
            match delta["type"].as_str() {
                Some("text_delta") => Ok(StreamChunk::TextDelta(
                    delta["text"].as_str().unwrap_or("").to_string(),
                )),
                Some("input_json_delta") => Ok(StreamChunk::ToolCallDelta {
                    index: parsed["index"].as_u64().unwrap_or(0) as usize,
                    args_delta: delta["partial_json"].as_str().unwrap_or("").to_string(),
                }),
                Some("thinking_delta") => Ok(StreamChunk::ThinkingDelta(
                    delta["thinking"].as_str().unwrap_or("").to_string(),
                )),
                _ => Ok(StreamChunk::TextDelta(String::new())),
            }
        }
        "ping" => Ok(StreamChunk::TextDelta(String::new())),
        _ => Ok(StreamChunk::TextDelta(String::new())),
    }
}
