use std::process::Command;

/// Hook event types that can trigger hook execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookEvent {
    PreToolUse,
    PostToolUse,
    PostToolUseFailure,
    AgentStart,
    AgentStop,
    UserPromptSubmit,
}

impl HookEvent {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PreToolUse" | "pre_tool_use" => Some(Self::PreToolUse),
            "PostToolUse" | "post_tool_use" => Some(Self::PostToolUse),
            "PostToolUseFailure" | "post_tool_use_failure" => Some(Self::PostToolUseFailure),
            "AgentStart" | "agent_start" => Some(Self::AgentStart),
            "AgentStop" | "agent_stop" => Some(Self::AgentStop),
            "UserPromptSubmit" | "user_prompt_submit" => Some(Self::UserPromptSubmit),
            _ => None,
        }
    }
}

/// A configured hook with event, optional tool matcher, and shell command.
#[derive(Debug, Clone)]
pub struct Hook {
    pub event: HookEvent,
    pub matcher: Option<String>,
    pub command: String,
    pub timeout_secs: u64,
}

impl Hook {
    pub fn new(event: HookEvent, command: String) -> Self {
        Self {
            event,
            matcher: None,
            command,
            timeout_secs: 30,
        }
    }

    pub fn with_matcher(mut self, matcher: &str) -> Self {
        self.matcher = Some(matcher.to_string());
        self
    }
}

/// Registry of configured hooks, executed by the agent at lifecycle events.
pub struct HookRegistry {
    hooks: Vec<Hook>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    pub fn register(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }

    /// Run all hooks matching the given event and optional tool name.
    pub fn run_hooks(&self, event: &HookEvent, tool_name: Option<&str>) {
        for hook in &self.hooks {
            if &hook.event != event {
                continue;
            }
            if let Some(matcher) = &hook.matcher {
                if let Some(name) = tool_name {
                    if name != matcher.as_str() {
                        continue;
                    }
                }
            }
            run_command_hook(hook, tool_name);
        }
    }
}

fn run_command_hook(hook: &Hook, tool_name: Option<&str>) {
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", &hook.command]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", &hook.command]);
        c
    };

    if let Some(name) = tool_name {
        cmd.env("HOOK_TOOL_NAME", name);
    }

    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                tracing::warn!("Hook {:?} failed: {}", hook.event, stderr.trim());
            }
        }
        Err(e) => {
            tracing::warn!("Hook {:?} error: {}", hook.event, e);
        }
    }
}
