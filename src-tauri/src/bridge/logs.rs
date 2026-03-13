//! Logs bridge for desktop UI.
//!
//! Provides runtime event logs for monitoring and debugging.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Log entry for UI display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

/// Log level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// In-memory log buffer (circular, max 1000 entries).
static LOG_BUFFER: LazyLock<RwLock<Vec<LogEntry>>> = LazyLock::new(|| RwLock::new(Vec::new()));
const MAX_LOG_ENTRIES: usize = 1000;

/// Generate a unique log ID.
#[allow(dead_code)]
fn generate_log_id() -> String {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64);
    format!("log-{now_ms}")
}

/// Format timestamp for display.
#[allow(dead_code)]
fn format_timestamp() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Add a log entry to the buffer.
#[allow(dead_code)]
pub async fn log(level: LogLevel, source: &str, message: &str) {
    let entry = LogEntry {
        id: generate_log_id(),
        timestamp: format_timestamp(),
        level,
        source: source.to_string(),
        message: message.to_string(),
    };

    let mut buffer = LOG_BUFFER.write().await;
    buffer.push(entry);

    // Trim old entries if over limit
    if buffer.len() > MAX_LOG_ENTRIES {
        let excess = buffer.len() - MAX_LOG_ENTRIES;
        buffer.drain(0..excess);
    }
}

/// Get recent log entries.
///
/// # Arguments
/// * `limit` - Maximum number of entries to return (default: 100)
/// * `level` - Optional filter by log level
pub async fn tail_logs(limit: Option<usize>, level: Option<LogLevel>) -> Vec<LogEntry> {
    let buffer = LOG_BUFFER.read().await;
    let limit = limit.unwrap_or(100).min(MAX_LOG_ENTRIES);

    let filtered: Vec<LogEntry> = if let Some(filter_level) = level {
        buffer
            .iter()
            .filter(|entry| entry.level == filter_level)
            .cloned()
            .collect()
    } else {
        buffer.clone()
    };

    // Return most recent entries
    filtered.into_iter().rev().take(limit).collect()
}

/// Clear all logs.
pub async fn clear_logs() {
    let mut buffer = LOG_BUFFER.write().await;
    buffer.clear();
}

/// Add a startup log entry (for initialization).
#[allow(dead_code)]
pub async fn init_logs() {
    log(
        LogLevel::Info,
        "system",
        "ZeroClaw Desktop initialized",
    ).await;
}