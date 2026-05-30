use super::*;
use futures::StreamExt;
use reqwest::Client;

pub struct OpenAiProvider {
    pub id: String,
    pub base_url: String,
    pub api_key: String,
    pub default_model: String,
    client: Client,
}

impl OpenAiProvider {
    pub fn new(id: String, base_url: String, api_key: String, default_model: String) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        Self {
            id,
            base_url,
            api_key,
            default_model,
            client,
        }
    }

    fn endpoint(&self) -> String {
        format!("{}/chat/completions", self.base_url.trim_end_matches('/'))
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
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

        let tool_specs: Vec<serde_json::Value> = if tools.is_empty() {
            vec![]
        } else {
            tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect()
        };

        let body = serde_json::json!({
            "model": model,
            "messages": messages.iter().map(|m| {
                let mut msg = serde_json::json!({
                    "role": m.role,
                    "content": m.content,
                });
                if let Some(tc) = &m.tool_calls {
                    msg["tool_calls"] = tc.clone();
                }
                if let Some(tci) = &m.tool_call_id {
                    msg["tool_call_id"] = serde_json::json!(tci);
                }
                msg
            }).collect::<Vec<_>>(),
            "tools": if tool_specs.is_empty() { serde_json::json!([]) } else { serde_json::json!(tool_specs) },
            "stream": true,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        let resp = self
            .client
            .post(&self.endpoint())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| z_claw_core::ClawError::Provider {
                provider: self.id.clone(),
                message: e.to_string(),
            })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| String::new());
            return Err(z_claw_core::ClawError::Provider {
                provider: self.id.clone(),
                message: format!("HTTP {status}: {text}"),
            });
        }

        let byte_stream = resp.bytes_stream();
        let stream = byte_stream.map(|result: reqwest::Result<_>| {
            let bytes = result.map_err(|e| z_claw_core::ClawError::Http(e.to_string()))?;
            let text = String::from_utf8_lossy(&bytes);
            parse_sse_chunk(&text)
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
        matches!(feature, Feature::ToolCalling | Feature::Streaming)
    }
}

fn parse_sse_chunk(text: &str) -> Result<StreamChunk, z_claw_core::ClawError> {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("data: ") {
            continue;
        }
        let json_str = &line["data: ".len()..];
        if json_str == "[DONE]" {
            return Ok(StreamChunk::Done {
                finish_reason: Some("stop".into()),
            });
        }
        let parsed: serde_json::Value = serde_json::from_str(json_str)?;
        if let Some(choices) = parsed["choices"].as_array() {
            for choice in choices {
                if let Some(reason) = choice["finish_reason"].as_str() {
                    if !reason.is_empty() && reason != "null" {
                        return Ok(StreamChunk::Done {
                            finish_reason: Some(reason.into()),
                        });
                    }
                }
                let delta = &choice["delta"];
                if let Some(content) = delta["content"].as_str() {
                    if !content.is_empty() {
                        return Ok(StreamChunk::TextDelta(content.to_string()));
                    }
                }
                if let Some(tool_calls) = delta["tool_calls"].as_array() {
                    for tc in tool_calls {
                        let index = tc["index"].as_u64().unwrap_or(0) as usize;
                        if let Some(id) = tc["id"].as_str() {
                            let name = tc["function"]["name"].as_str().unwrap_or("");
                            return Ok(StreamChunk::ToolCallStart {
                                index,
                                id: id.to_string(),
                                name: name.to_string(),
                            });
                        }
                        if let Some(args) = tc["function"]["arguments"].as_str() {
                            if !args.is_empty() {
                                return Ok(StreamChunk::ToolCallDelta {
                                    index,
                                    args_delta: args.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(StreamChunk::TextDelta(String::new()))
}
