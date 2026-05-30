use super::Tool;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;

/// An MCP tool discovered from a server.
pub struct McpTool {
    server_name: String,
    tool_name: String,
    description: String,
    input_schema: serde_json::Value,
    command: String,
    args: Vec<String>,
}

impl McpTool {
    pub fn new(
        server_name: &str,
        command: &str,
        args: &[String],
        name: &str,
        desc: &str,
        schema: serde_json::Value,
    ) -> Self {
        Self {
            server_name: server_name.to_string(),
            tool_name: name.to_string(),
            description: desc.to_string(),
            input_schema: schema,
            command: command.to_string(),
            args: args.to_vec(),
        }
    }
}

#[async_trait]
impl Tool for McpTool {
    fn name(&self) -> &str {
        &self.tool_name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn parameters(&self) -> serde_json::Value {
        self.input_schema.clone()
    }

    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let args_refs: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
        spawn_and_call(&self.command, &args_refs, &self.tool_name, args)
            .await
            .map_err(|e| z_claw_core::ClawError::Tool {
                tool: self.name().to_string(),
                message: e,
            })
    }
}

async fn send_request(
    writer: &Arc<Mutex<tokio::process::ChildStdin>>,
    reader: impl tokio::io::AsyncRead + Unpin,
    id: u64,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let request = serde_json::json!({"jsonrpc":"2.0","id":id,"method":method,"params":params});
    let mut body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
    body.push(b'\n');
    writer
        .lock()
        .await
        .write_all(&body)
        .await
        .map_err(|e| e.to_string())?;

    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&line).map_err(|e| format!("Parse: {e}"))
}

async fn spawn_and_call(
    command: &str,
    args: &[&str],
    tool_name: &str,
    tool_args: serde_json::Value,
) -> Result<String, String> {
    let mut child = Command::new(command)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn: {e}"))?;

    let stdin = child.stdin.take().ok_or("no stdin")?;
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let writer = Arc::new(Mutex::new(stdin));
    let init_params = serde_json::json!({
        "protocolVersion": "2024-11-05", "capabilities": {},
        "clientInfo": {"name":"z-claw","version":"0.1.0"}
    });

    send_request(&writer, stdout, 0, "initialize", init_params).await?;
    let stdout2 = child.stdout.take().ok_or("no stdout after init")?;

    let response = send_request(
        &writer,
        stdout2,
        1,
        "tools/call",
        serde_json::json!({"name": tool_name, "arguments": tool_args}),
    )
    .await?;

    let _ = child.kill().await;

    if let Some(err) = response.get("error") {
        return Err(format!("MCP error: {}", err["message"]));
    }
    let content = &response["result"]["content"];
    if let Some(arr) = content.as_array() {
        Ok(arr
            .iter()
            .filter_map(|c| c["text"].as_str().map(|t| t.to_string()))
            .collect::<Vec<_>>()
            .join("\n"))
    } else {
        Ok(content.to_string())
    }
}

/// Discover tools from an MCP server via JSON-RPC/stdio.
pub async fn discover_mcp_tools(
    command: &str,
    args: &[&str],
) -> Result<Vec<(String, String, serde_json::Value)>, String> {
    let mut child = Command::new(command)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn: {e}"))?;

    let stdin = child.stdin.take().ok_or("no stdin")?;
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let writer = Arc::new(Mutex::new(stdin));
    let init_params = serde_json::json!({
        "protocolVersion": "2024-11-05", "capabilities": {},
        "clientInfo": {"name":"z-claw","version":"0.1.0"}
    });

    send_request(&writer, stdout, 0, "initialize", init_params).await?;
    let stdout2 = child.stdout.take().ok_or("no stdout after init")?;

    let response = send_request(&writer, stdout2, 1, "tools/list", serde_json::json!({})).await?;
    let _ = child.kill().await;

    Ok(response["result"]["tools"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|t| {
                    (
                        t["name"].as_str().unwrap_or("").to_string(),
                        t["description"].as_str().unwrap_or("").to_string(),
                        t["inputSchema"].clone(),
                    )
                })
                .collect()
        })
        .unwrap_or_default())
}
