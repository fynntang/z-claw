use std::process::Command;

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

#[derive(Debug, Clone)]
pub enum HookType {
    Command(String),
    Http { url: String, method: String },
    Prompt(String),
}

#[derive(Debug, Clone)]
pub struct Hook {
    pub event: HookEvent,
    pub matcher: Option<String>,
    pub hook_type: HookType,
    pub timeout_secs: u64,
}

impl Hook {
    pub fn new(event: HookEvent, command: String) -> Self {
        Self {
            event,
            matcher: None,
            hook_type: HookType::Command(command),
            timeout_secs: 30,
        }
    }

    pub fn http(event: HookEvent, url: &str, method: &str) -> Self {
        Self {
            event,
            matcher: None,
            hook_type: HookType::Http {
                url: url.to_string(),
                method: method.to_string(),
            },
            timeout_secs: 30,
        }
    }

    pub fn prompt(event: HookEvent, prompt: &str) -> Self {
        Self {
            event,
            matcher: None,
            hook_type: HookType::Prompt(prompt.to_string()),
            timeout_secs: 30,
        }
    }

    pub fn with_matcher(mut self, matcher: &str) -> Self {
        self.matcher = Some(matcher.to_string());
        self
    }
}

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
            execute_hook(hook, tool_name);
        }
    }
}

fn execute_hook(hook: &Hook, tool_name: Option<&str>) {
    match &hook.hook_type {
        HookType::Command(cmd) => {
            let mut child = if cfg!(target_os = "windows") {
                let mut c = Command::new("cmd");
                c.args(["/C", cmd]);
                c
            } else {
                let mut c = Command::new("sh");
                c.args(["-c", cmd]);
                c
            };
            if let Some(name) = tool_name {
                child.env("HOOK_TOOL_NAME", name);
            }
            match child.output() {
                Ok(o) => {
                    if !o.status.success() {
                        tracing::warn!(
                            "Hook cmd failed: {}",
                            String::from_utf8_lossy(&o.stderr).trim()
                        );
                    }
                }
                Err(e) => tracing::warn!("Hook cmd error: {e}"),
            }
        }
        HookType::Http { url, method } => {
            // HTTP hooks are spawned as fire-and-forget
            let url = url.clone();
            let method = method.clone();
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let req = match method.to_uppercase().as_str() {
                    "POST" => client.post(&url),
                    _ => client.get(&url),
                };
                match req.send().await {
                    Ok(r) => tracing::info!("Hook HTTP {} {} → {}", method, url, r.status()),
                    Err(e) => tracing::warn!("Hook HTTP error: {e}"),
                }
            });
        }
        HookType::Prompt(prompt) => {
            tracing::info!("Hook prompt: {prompt}");
            // Prompt hooks are informational — the prompt text is logged for agent context
        }
    }
}
