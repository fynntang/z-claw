use serde::{Deserialize, Serialize};

/// 5-tier security classification for tool calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    AutoExecute = 0,
    ConfirmExecute = 1,
    SandboxExecute = 2,
    RequireApproval = 3,
    Blocked = 4,
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityLevel::AutoExecute => write!(f, "auto_execute"),
            SecurityLevel::ConfirmExecute => write!(f, "confirm_execute"),
            SecurityLevel::SandboxExecute => write!(f, "sandbox_execute"),
            SecurityLevel::RequireApproval => write!(f, "require_approval"),
            SecurityLevel::Blocked => write!(f, "blocked"),
        }
    }
}

impl SecurityLevel {
    pub fn from_str(s: &str) -> Self {
        match s {
            "auto_execute" => SecurityLevel::AutoExecute,
            "confirm_execute" => SecurityLevel::ConfirmExecute,
            "sandbox_execute" => SecurityLevel::SandboxExecute,
            "require_approval" => SecurityLevel::RequireApproval,
            "blocked" => SecurityLevel::Blocked,
            _ => SecurityLevel::ConfirmExecute,
        }
    }
}

pub struct PolicyEngine {
    blocked_commands: Vec<String>,
    allowed_path_prefixes: Vec<String>,
    default_level: SecurityLevel,
}

impl PolicyEngine {
    pub fn new(
        blocked_commands: Vec<String>,
        allowed_path_prefixes: Vec<String>,
        default_level: SecurityLevel,
    ) -> Self {
        Self { blocked_commands, allowed_path_prefixes, default_level }
    }

    pub fn classify(&self, tool_name: &str, args: &serde_json::Value) -> SecurityLevel {
        match tool_name {
            "read_file" | "list_directory" | "search_code"
            | "search_memory" | "read_memory" => SecurityLevel::AutoExecute,

            "write_file" | "create_directory" => SecurityLevel::ConfirmExecute,

            "execute_command" => {
                if self.is_safe_command(args) {
                    SecurityLevel::ConfirmExecute
                } else {
                    SecurityLevel::SandboxExecute
                }
            }

            "http_request" | "git_push" | "browser_navigate"
            | "git_operation" => SecurityLevel::RequireApproval,

            "system_config" | "install_package" | "uninstall_package"
            | "modify_registry" => SecurityLevel::Blocked,

            _ => self.default_level,
        }
    }

    fn is_safe_command(&self, args: &serde_json::Value) -> bool {
        let cmd = args.get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        !self.blocked_commands.iter().any(|blocked| cmd.contains(blocked.as_str()))
    }

    pub fn is_path_allowed(&self, path: &str) -> bool {
        if self.allowed_path_prefixes.is_empty() {
            return true;
        }
        self.allowed_path_prefixes.iter().any(|prefix| path.starts_with(prefix))
    }

    pub fn needs_approval(&self, level: SecurityLevel) -> bool {
        level >= SecurityLevel::RequireApproval
    }

    pub fn is_blocked(&self, level: SecurityLevel) -> bool {
        level == SecurityLevel::Blocked
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp_ms: i64,
    pub session_id: String,
    pub tool_name: String,
    pub arguments_json: String,
    pub security_level: String,
    pub approved: bool,
    pub success: bool,
    pub summary: String,
}
