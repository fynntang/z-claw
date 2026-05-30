use std::sync::Mutex;

use async_trait::async_trait;
use rusqlite::Connection;
use z_claw_core::{ClawError, HistoryMessage};

use crate::MemoryBackend;
use crate::embedding;

const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    created_ms INTEGER NOT NULL,
    updated_ms INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    tool_calls TEXT,
    created_ms INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE TABLE IF NOT EXISTS knowledge (
    id TEXT PRIMARY KEY,
    memory_type TEXT NOT NULL DEFAULT 'reference',
    title TEXT NOT NULL,
    body TEXT NOT NULL DEFAULT '',
    embedding BLOB,
    created_ms INTEGER NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_fts USING fts5(
    title, body, content='knowledge', content_rowid='rowid'
);

CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, created_ms);

";

/// SQLite-backed implementation of MemoryBackend.
pub struct SqliteMemory {
    db: Mutex<Connection>,
}

impl SqliteMemory {
    pub fn new(db_path: impl Into<std::path::PathBuf>) -> Result<Self, ClawError> {
        let path: std::path::PathBuf = db_path.into();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ClawError::Io(e))?;
        }
        let conn = Connection::open(&path).map_err(|e| ClawError::Sqlite(e.to_string()))?;
        conn.execute_batch(SCHEMA_SQL)
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(Self {
            db: Mutex::new(conn),
        })
    }
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[async_trait]
impl MemoryBackend for SqliteMemory {
    async fn append_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
        tool_calls: Option<serde_json::Value>,
    ) -> Result<(), ClawError> {
        let id = uuid::Uuid::new_v4().to_string();
        let ts = now_ms();
        let tc_json = tool_calls
            .as_ref()
            .map(|v| serde_json::to_string(v))
            .transpose()
            .map_err(|e| ClawError::Serde(e))?;
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "INSERT INTO messages (id, session_id, role, content, tool_calls, created_ms) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, session_id, role, content, tc_json, ts],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "UPDATE sessions SET updated_ms = ?1 WHERE id = ?2",
            rusqlite::params![ts, session_id],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(())
    }

    async fn load_recent(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<HistoryMessage>, ClawError> {
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let mut stmt = db
            .prepare(
                "SELECT id, session_id, role, content, tool_calls, created_ms FROM messages \
                 WHERE session_id = ?1 ORDER BY created_ms DESC LIMIT ?2",
            )
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let rows = stmt
            .query_map(rusqlite::params![session_id, limit as i64], |row| {
                Ok(HistoryMessage {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    tool_calls: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| serde_json::from_str(&s).ok()),
                    created_ms: row.get(5)?,
                })
            })
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let mut messages: Vec<HistoryMessage> = Vec::new();
        for row in rows {
            messages.push(row.map_err(|e| ClawError::Sqlite(e.to_string()))?);
        }
        messages.reverse();
        Ok(messages)
    }

    async fn list_sessions(&self) -> Result<Vec<(String, String, i64)>, ClawError> {
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let mut stmt = db
            .prepare("SELECT id, title, updated_ms FROM sessions ORDER BY updated_ms DESC")
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row.map_err(|e| ClawError::Sqlite(e.to_string()))?);
        }
        Ok(sessions)
    }

    async fn create_session(&self, id: &str, title: &str) -> Result<(), ClawError> {
        let ts = now_ms();
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "INSERT OR IGNORE INTO sessions (id, title, created_ms, updated_ms) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, title, ts, ts],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(())
    }

    async fn delete_session(&self, id: &str) -> Result<(), ClawError> {
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "DELETE FROM messages WHERE session_id = ?1",
            rusqlite::params![id],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute("DELETE FROM sessions WHERE id = ?1", rusqlite::params![id])
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(())
    }

    async fn rename_session(&self, id: &str, title: &str) -> Result<(), ClawError> {
        let ts = now_ms();
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "UPDATE sessions SET title = ?1, updated_ms = ?2 WHERE id = ?3",
            rusqlite::params![title, ts, id],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(())
    }

    async fn store_knowledge(
        &self,
        memory_type: &str,
        title: &str,
        body: &str,
    ) -> Result<String, ClawError> {
        let id = uuid::Uuid::new_v4().to_string();
        let ts = now_ms();

        // Try embedding the content for semantic search
        let emb_blob = embedding::get_embedding(&format!("{title}: {body}"))
            .await
            .ok()
            .map(|v| embedding::encode_vector(&v));

        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute(
            "INSERT INTO knowledge (id, memory_type, title, body, embedding, created_ms) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, memory_type, title, body, emb_blob, ts],
        )
        .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        // Sync to FTS index
        let _ = db.execute(
            "INSERT INTO knowledge_fts(rowid, title, body) VALUES (last_insert_rowid(), ?1, ?2)",
            rusqlite::params![title, body],
        );
        Ok(id)
    }

    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<String>, ClawError> {
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        // Try FTS5 first, fall back to LIKE
        let mut stmt = db
            .prepare(
                "SELECT k.memory_type, k.title, k.body FROM knowledge k \
                 JOIN knowledge_fts fts ON k.rowid = fts.rowid \
                 WHERE knowledge_fts MATCH ?1 \
                 ORDER BY rank LIMIT ?2",
            )
            .map_err(|_| {
                // FTS may not exist yet, will fall back below
            });
        let results = match stmt {
            Ok(ref mut s) => {
                let rows = s
                    .query_map(rusqlite::params![query, limit as i64], |row| {
                        Ok(format!(
                            "[{}] {}: {}",
                            row.get::<_, String>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, String>(2)?
                        ))
                    })
                    .map_err(|e| ClawError::Sqlite(e.to_string()))?;
                let mut v = Vec::new();
                for row in rows {
                    v.push(row.map_err(|e| ClawError::Sqlite(e.to_string()))?);
                }
                v
            }
            Err(_) => {
                // Fallback to LIKE
                let pattern = format!("%{}%", query);
                let mut s = db
                    .prepare(
                        "SELECT memory_type, title, body FROM knowledge \
                     WHERE title LIKE ?1 OR body LIKE ?1 \
                     ORDER BY created_ms DESC LIMIT ?2",
                    )
                    .map_err(|e| ClawError::Sqlite(e.to_string()))?;
                let rows = s
                    .query_map(rusqlite::params![pattern, limit as i64], |row| {
                        Ok(format!(
                            "[{}] {}: {}",
                            row.get::<_, String>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, String>(2)?
                        ))
                    })
                    .map_err(|e| ClawError::Sqlite(e.to_string()))?;
                let mut v = Vec::new();
                for row in rows {
                    v.push(row.map_err(|e| ClawError::Sqlite(e.to_string()))?);
                }
                v
            }
        };
        Ok(results)
    }

    async fn forget_knowledge(&self, id: &str) -> Result<(), ClawError> {
        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        db.execute("DELETE FROM knowledge WHERE id = ?1", rusqlite::params![id])
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        Ok(())
    }

    async fn search_semantic(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<(String, f32)>, ClawError> {
        // Get embedding for the query
        let q_vec = embedding::get_embedding(query)
            .await
            .map_err(|e| ClawError::Sqlite(format!("Embedding error: {e}")))?;

        let db = self
            .db
            .lock()
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;
        let mut stmt = db
            .prepare("SELECT title, embedding FROM knowledge WHERE embedding IS NOT NULL")
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;

        let mut results: Vec<(String, f32)> = Vec::new();
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?))
            })
            .map_err(|e| ClawError::Sqlite(e.to_string()))?;

        for row in rows {
            let (title, emb_bytes) = row.map_err(|e| ClawError::Sqlite(e.to_string()))?;
            let emb = embedding::decode_vector(&emb_bytes);
            let sim = embedding::cosine_similarity(&q_vec, &emb);
            results.push((title, sim));
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }
}
