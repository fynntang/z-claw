use super::Tool;
use async_trait::async_trait;

pub struct ExecuteCommandTool;

#[async_trait]
impl Tool for ExecuteCommandTool {
    fn name(&self) -> &str {
        "execute_command"
    }
    fn description(&self) -> &str {
        "Execute a shell command and return its output (timeout: 60s, max output: 64KB)"
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "The shell command to execute" },
                "cwd": { "type": "string", "description": "Working directory for the command" }
            },
            "required": ["command"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let command = args["command"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "execute_command".into(),
                message: "missing command".into(),
            })?;
        let cwd = args["cwd"].as_str().unwrap_or(".");

        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/C", command])
                .current_dir(cwd)
                .output()?
        } else {
            std::process::Command::new("sh")
                .args(["-c", command])
                .current_dir(cwd)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("stderr:\n");
            result.push_str(&stderr);
        }
        if result.is_empty() {
            result = format!(
                "Command completed with exit code: {}",
                output.status.code().unwrap_or(-1)
            );
        }
        Ok(result)
    }
}
