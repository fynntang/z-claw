mod filesystem;
mod shell;

pub use filesystem::{ReadFileTool, WriteFileTool, ListDirectoryTool};
pub use shell::ExecuteCommandTool;

use async_trait::async_trait;
use std::sync::Arc;
use z_claw_core::ToolDef;

/// A tool that can be called by the agent.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value) -> Result<String, z_claw_core::ClawError>;
}

/// Registry of available tools.
pub struct ToolRegistry {
    tools: Vec<Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.iter().find(|t| t.name() == name).cloned()
    }

    pub fn definitions(&self) -> Vec<ToolDef> {
        self.tools.iter().map(|t| ToolDef {
            name: t.name().to_string(),
            description: t.description().to_string(),
            parameters: t.parameters(),
        }).collect()
    }

    pub fn names(&self) -> Vec<String> {
        self.tools.iter().map(|t| t.name().to_string()).collect()
    }
}

/// Register all built-in tools.
pub fn builtin_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(filesystem::ReadFileTool));
    registry.register(Arc::new(filesystem::WriteFileTool));
    registry.register(Arc::new(filesystem::ListDirectoryTool));
    registry.register(Arc::new(shell::ExecuteCommandTool));
    registry
}
