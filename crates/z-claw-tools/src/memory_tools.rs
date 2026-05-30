use super::Tool;
use async_trait::async_trait;
use std::sync::Arc;
use z_claw_memory::MemoryBackend;

pub struct StoreKnowledgeTool {
    memory: Arc<dyn MemoryBackend>,
}

impl StoreKnowledgeTool {
    pub fn new(memory: Arc<dyn MemoryBackend>) -> Self {
        Self { memory }
    }
}

#[async_trait]
impl Tool for StoreKnowledgeTool {
    fn name(&self) -> &str {
        "store_knowledge"
    }
    fn description(&self) -> &str {
        "Store knowledge for future reference. Types: user, feedback, project, reference."
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "memory_type": {
                    "type": "string", "description": "Category",
                    "enum": ["user", "feedback", "project", "reference"]
                },
                "title": { "type": "string", "description": "Short title" },
                "body": { "type": "string", "description": "Content to store" }
            },
            "required": ["memory_type", "title", "body"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let mem_type = args["memory_type"].as_str().unwrap_or("reference");
        let title = args["title"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "store_knowledge".into(),
                message: "missing title".into(),
            })?;
        let body = args["body"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "store_knowledge".into(),
                message: "missing body".into(),
            })?;
        let id = self.memory.store_knowledge(mem_type, title, body).await?;
        Ok(format!("Knowledge stored with id: {id}"))
    }
}

pub struct SearchMemoryTool {
    memory: Arc<dyn MemoryBackend>,
}

impl SearchMemoryTool {
    pub fn new(memory: Arc<dyn MemoryBackend>) -> Self {
        Self { memory }
    }
}

#[async_trait]
impl Tool for SearchMemoryTool {
    fn name(&self) -> &str {
        "search_memory"
    }
    fn description(&self) -> &str {
        "Search stored knowledge. Returns matching entries with type, title, and body."
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search query" },
                "limit": { "type": "integer", "description": "Max results (default 10)" }
            },
            "required": ["query"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "search_memory".into(),
                message: "missing query".into(),
            })?;
        let limit = args["limit"].as_u64().unwrap_or(10) as usize;
        let results = self.memory.search_knowledge(query, limit).await?;
        if results.is_empty() {
            Ok("No matching knowledge found.".into())
        } else {
            Ok(results.join("\n---\n"))
        }
    }
}

pub struct ForgetKnowledgeTool {
    memory: Arc<dyn MemoryBackend>,
}

impl ForgetKnowledgeTool {
    pub fn new(memory: Arc<dyn MemoryBackend>) -> Self {
        Self { memory }
    }
}

#[async_trait]
impl Tool for ForgetKnowledgeTool {
    fn name(&self) -> &str {
        "forget_knowledge"
    }
    fn description(&self) -> &str {
        "Delete a knowledge entry by its ID."
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Knowledge entry ID to delete" }
            },
            "required": ["id"]
        })
    }
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError> {
        let id = args["id"]
            .as_str()
            .ok_or_else(|| z_claw_core::ClawError::Tool {
                tool: "forget_knowledge".into(),
                message: "missing id".into(),
            })?;
        self.memory.forget_knowledge(id).await?;
        Ok(format!("Knowledge entry {id} deleted."))
    }
}
