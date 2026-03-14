//! Workspace bridge: read ZeroClaw workspace directory and key files (IDENTITY.md, USER.md, etc.).
//! 使用本地路径 ~/.zeroclaw/workspace，不依赖 zeroclaw 内部 API。

use serde::Serialize;
use std::path::PathBuf;

/// ZeroClaw workspace 根目录（与 zeroclaw 约定一致：~/.zeroclaw/workspace）。
fn get_workspace_dir() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".zeroclaw").join("workspace"))
        .unwrap_or_else(|| PathBuf::from(".").join("zeroclaw").join("workspace"))
}

/// One entry in a directory listing.
#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceEntry {
    pub name: String,
    pub is_dir: bool,
}

/// Returns the workspace root path as a string (for display).
pub fn workspace_path() -> String {
    get_workspace_dir().display().to_string()
}

fn normalize_relative(rel: &str) -> Result<String, String> {
    let s = rel.trim().replace('\\', "/");
    if s.contains("..") || s.starts_with('/') {
        return Err("Path is outside workspace".to_string());
    }
    Ok(s)
}

/// List directory contents under workspace. `subpath` is relative to workspace root (e.g. "" or "memory").
/// Fails if subpath escapes workspace.
pub fn workspace_list_dir(subpath: Option<String>) -> Result<Vec<WorkspaceEntry>, String> {
    let root = get_workspace_dir();
    let path = match &subpath {
        None => root.clone(),
        Some(s) if s.trim().is_empty() => root.clone(),
        Some(s) => {
            let s = normalize_relative(s)?;
            root.join(s)
        }
    };

    if !path.exists() {
        return Ok(Vec::new());
    }
    if !path.is_dir() {
        return Err("Not a directory".to_string());
    }

    let mut entries: Vec<WorkspaceEntry> = std::fs::read_dir(&path)
        .map_err(|e| format!("Read dir failed: {}", e))?
        .filter_map(|e| e.ok())
        .map(|e| {
            let is_dir = e.path().is_dir();
            let name = e.file_name().to_string_lossy().to_string();
            WorkspaceEntry { name, is_dir }
        })
        .collect();
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    Ok(entries)
}

/// Read a file under workspace. `relative_path` is relative to workspace root (e.g. "IDENTITY.md", "memory/2025-01-01.md").
pub fn workspace_read_file(relative_path: String) -> Result<String, String> {
    let rel = normalize_relative(&relative_path)?;
    let root = get_workspace_dir();
    let path = root.join(&rel);
    if !path.exists() || !path.is_file() {
        return Err("File not found".to_string());
    }
    let root_canonical = root.canonicalize().unwrap_or_else(|_| root.clone());
    let path_canonical = path.canonicalize().map_err(|e| format!("Invalid path: {}", e))?;
    if !path_canonical.starts_with(&root_canonical) {
        return Err("Path is outside workspace".to_string());
    }
    std::fs::read_to_string(&path).map_err(|e| format!("Read file failed: {}", e))
}
