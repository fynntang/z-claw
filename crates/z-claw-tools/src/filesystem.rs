use super::Tool;
use async_trait::async_trait;

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str { "read_file" }
    fn description(&self) -> &str { "Read the contents of a file at the given path" }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the file to read" }
            },
            "required": ["path"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let path = args["path"].as_str().ok_or_else(|| {
            z_claw_core::ClawError::Tool { tool: "read_file".into(), message: "missing path".into() }
        })?;
        Ok(std::fs::read_to_string(path)?)
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to a file, creating parent directories if needed" }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the file to write" },
                "content": { "type": "string", "description": "Content to write to the file" }
            },
            "required": ["path", "content"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let path = args["path"].as_str().ok_or_else(|| {
            z_claw_core::ClawError::Tool { tool: "write_file".into(), message: "missing path".into() }
        })?;
        let content = args["content"].as_str().ok_or_else(|| {
            z_claw_core::ClawError::Tool { tool: "write_file".into(), message: "missing content".into() }
        })?;
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(format!("File written: {path}"))
    }
}

pub struct ListDirectoryTool;

#[async_trait]
impl Tool for ListDirectoryTool {
    fn name(&self) -> &str { "list_directory" }
    fn description(&self) -> &str { "List the contents of a directory" }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the directory to list" }
            },
            "required": ["path"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let path = args["path"].as_str().ok_or_else(|| {
            z_claw_core::ClawError::Tool { tool: "list_directory".into(), message: "missing path".into() }
        })?;
        let entries: Vec<String> = std::fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .map(|e| {
                let file_type = e.file_type().ok().map(|t| if t.is_dir() { "d" } else { "f" }.to_string()).unwrap_or_else(|| "?".into());
                format!("{} {}", file_type, e.file_name().to_string_lossy())
            })
            .collect();
        Ok(entries.join("\n"))
    }
}
