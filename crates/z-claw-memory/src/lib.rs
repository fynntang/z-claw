use async_trait::async_trait;
use z_claw_core::HistoryMessage;

pub mod embedding;
mod sqlite;

pub use sqlite::SqliteMemory;

/// Persistent memory backend for sessions, messages, and knowledge.
#[async_trait]
pub trait MemoryBackend: Send + Sync {
    async fn append_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
        tool_calls: Option<serde_json::Value>,
    ) -> Result<(), z_claw_core::ClawError>;
    async fn load_recent(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<HistoryMessage>, z_claw_core::ClawError>;
    async fn list_sessions(&self) -> Result<Vec<(String, String, i64)>, z_claw_core::ClawError>;
    async fn create_session(&self, id: &str, title: &str) -> Result<(), z_claw_core::ClawError>;
    async fn delete_session(&self, id: &str) -> Result<(), z_claw_core::ClawError>;
    async fn rename_session(&self, id: &str, title: &str) -> Result<(), z_claw_core::ClawError>;
    async fn store_knowledge(
        &self,
        memory_type: &str,
        title: &str,
        body: &str,
    ) -> Result<String, z_claw_core::ClawError>;
    async fn search_knowledge(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<String>, z_claw_core::ClawError>;
    async fn forget_knowledge(&self, id: &str) -> Result<(), z_claw_core::ClawError>;
    async fn search_semantic(
        &self,
        _query: &str,
        _limit: usize,
    ) -> Result<Vec<(String, f32)>, z_claw_core::ClawError> {
        Ok(vec![])
    }
}

/// A no-op memory backend for MVP (no persistence).
pub struct NoopMemory;

#[async_trait]
impl MemoryBackend for NoopMemory {
    async fn append_message(
        &self,
        _session_id: &str,
        _role: &str,
        _content: &str,
        _tool_calls: Option<serde_json::Value>,
    ) -> Result<(), z_claw_core::ClawError> {
        Ok(())
    }
    async fn load_recent(
        &self,
        _session_id: &str,
        _limit: usize,
    ) -> Result<Vec<HistoryMessage>, z_claw_core::ClawError> {
        Ok(vec![])
    }
    async fn list_sessions(&self) -> Result<Vec<(String, String, i64)>, z_claw_core::ClawError> {
        Ok(vec![])
    }
    async fn create_session(&self, _id: &str, _title: &str) -> Result<(), z_claw_core::ClawError> {
        Ok(())
    }
    async fn delete_session(&self, _id: &str) -> Result<(), z_claw_core::ClawError> {
        Ok(())
    }
    async fn rename_session(&self, _id: &str, _title: &str) -> Result<(), z_claw_core::ClawError> {
        Ok(())
    }
    async fn store_knowledge(
        &self,
        _memory_type: &str,
        _title: &str,
        _body: &str,
    ) -> Result<String, z_claw_core::ClawError> {
        Ok("noop".into())
    }
    async fn search_knowledge(
        &self,
        _query: &str,
        _limit: usize,
    ) -> Result<Vec<String>, z_claw_core::ClawError> {
        Ok(vec![])
    }
    async fn forget_knowledge(&self, _id: &str) -> Result<(), z_claw_core::ClawError> {
        Ok(())
    }
}
