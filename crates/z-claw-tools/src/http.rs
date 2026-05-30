use super::Tool;
use async_trait::async_trait;

pub struct HttpRequestTool;

#[async_trait]
impl Tool for HttpRequestTool {
    fn name(&self) -> &str {
        "http_request"
    }
    fn description(&self) -> &str {
        "Make an HTTP request. Returns status code, headers, and body (max 64KB)."
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string", "description": "HTTP method",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": { "type": "string", "description": "The URL to request" },
                "headers": {
                    "type": "object", "description": "Optional headers as key-value pairs"
                },
                "body": { "type": "string", "description": "Optional request body" }
            },
            "required": ["method", "url"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let method = args["method"].as_str().unwrap_or("GET").to_uppercase();
        let url = args["url"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "http_request".into(),
                message: "missing url".into(),
            })?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| z_claw_core::ClawError::Tool {
                tool: "http_request".into(),
                message: e.to_string(),
            })?;

        let mut req = match method.as_str() {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => client.get(url),
        };

        if let Some(headers) = args["headers"].as_object() {
            for (k, v) in headers {
                if let Some(val) = v.as_str() {
                    req = req.header(k.as_str(), val);
                }
            }
        }
        if let Some(b) = args["body"].as_str() {
            req = req.body(b.to_string());
        }

        let resp = req.send().await.map_err(|e| z_claw_core::ClawError::Tool {
            tool: "http_request".into(),
            message: e.to_string(),
        })?;

        let status = resp.status();
        let resp_headers = resp
            .headers()
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("?")))
            .collect::<Vec<_>>()
            .join("\n");

        let body_text = resp.text().await.unwrap_or_default();
        let truncated = if body_text.len() > 65536 {
            format!("{}... (truncated)", &body_text[..65536])
        } else {
            body_text
        };

        Ok(format!(
            "HTTP {} {}\n\nHeaders:\n{}\n\nBody:\n{}",
            status.as_u16(),
            status.canonical_reason().unwrap_or(""),
            resp_headers,
            truncated
        ))
    }
}
