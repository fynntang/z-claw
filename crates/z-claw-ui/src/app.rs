use crate::views::approval::ApprovalRequest;
use gpui::*;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use z_claw_agent::{AgentLoop, Harness, HookRegistry, default_system_prompt};
use z_claw_core::AgentEvent;
use z_claw_core::{NativePlatform, Platform};
use z_claw_memory::{MemoryBackend, SqliteMemory};
use z_claw_providers::{AnthropicProvider, LlmProvider, OpenAiProvider, ProviderChain};
use z_claw_security::{PolicyEngine, SecurityLevel};
use z_claw_skills::SkillRegistry;
use z_claw_tools::builtin_tools;

pub struct AppModel {
    pub agent: Option<AgentLoop>,
    pub memory: Arc<dyn MemoryBackend>,
    pub event_rx: mpsc::UnboundedReceiver<AgentEvent>,
    pub event_tx: mpsc::UnboundedSender<AgentEvent>,
    pub messages: Vec<MessageItem>,
    pub input_text: String,
    pub streaming: bool,
    pub session_id: String,
    pub pending_approval: Option<ApprovalRequest>,
    pub approval_tx: Arc<Mutex<Option<tokio::sync::oneshot::Sender<bool>>>>,
}

#[derive(Debug, Clone)]
pub struct MessageItem {
    pub role: String,
    pub content: String,
    pub tool_calls: Vec<ToolCallItem>,
}

#[derive(Debug, Clone)]
pub struct ToolCallItem {
    pub name: String,
    pub status: String,
    pub summary: String,
}

impl AppModel {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let platform = NativePlatform;
        let data_dir = platform.data_dir();
        std::fs::create_dir_all(&data_dir).ok();

        let memory: Arc<dyn MemoryBackend> = Arc::new(
            SqliteMemory::new(data_dir.join("sessions.db"))
                .expect("failed to create SQLite memory backend"),
        );

        // Build provider chain: local first, then cloud fallbacks
        let mut providers: Vec<Arc<dyn LlmProvider>> = vec![
            // Primary: Ollama (local, always available)
            Arc::new(OpenAiProvider::new(
                "ollama".into(),
                "http://localhost:11434/v1".into(),
                "ollama".into(),
                "llama3".into(),
            )),
        ];

        // OpenAI if API key is set
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            if !key.is_empty() {
                providers.push(Arc::new(OpenAiProvider::new(
                    "openai".into(),
                    "https://api.openai.com/v1".into(),
                    key,
                    "gpt-4o".into(),
                )));
            }
        }

        // DeepSeek if API key is set
        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            if !key.is_empty() {
                providers.push(Arc::new(OpenAiProvider::new(
                    "deepseek".into(),
                    "https://api.deepseek.com/v1".into(),
                    key,
                    "deepseek-chat".into(),
                )));
            }
        }

        // Anthropic if API key is set
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                providers.push(Arc::new(AnthropicProvider::new(
                    "anthropic".into(),
                    key,
                    "claude-sonnet-4-6".into(),
                )));
            }
        }

        let chain = ProviderChain::new(providers);

        let harness = Arc::new(Harness {
            providers: chain,
            tools: Arc::new(builtin_tools(Some(memory.clone()))),
            memory: memory.clone(),
            policy: PolicyEngine::new(vec![], vec!["~".into()], SecurityLevel::ConfirmExecute),
            system_prompt: default_system_prompt(),
            hooks: HookRegistry::new(),
            skills: Arc::new(SkillRegistry::new()),
        });

        let session_id = uuid::Uuid::new_v4().to_string();
        let agent = AgentLoop::new(harness, session_id.clone());

        Self {
            agent: Some(agent),
            memory,
            event_rx,
            event_tx,
            messages: Vec::new(),
            input_text: String::new(),
            streaming: false,
            session_id,
            pending_approval: None,
            approval_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Approve the pending tool (called from UI).
    pub fn approve(&self) {
        if let Some(tx) = self.approval_tx.try_lock().ok().and_then(|mut g| g.take()) {
            let _ = tx.send(true);
        }
    }

    /// Deny the pending tool (called from UI).
    pub fn deny(&self) {
        if let Some(tx) = self.approval_tx.try_lock().ok().and_then(|mut g| g.take()) {
            let _ = tx.send(false);
        }
    }

    /// Clear the pending tool approval (called after user decides).
    pub fn clear_approval(&mut self) {
        self.pending_approval = None;
    }

    /// Start a fresh session with a new agent loop.
    pub fn new_session(&mut self) {
        self.messages.clear();
        self.streaming = false;
        self.input_text.clear();
        let session_id = uuid::Uuid::new_v4().to_string();
        self.session_id = session_id.clone();
        let harness = Arc::new(Harness {
            providers: ProviderChain::from_single(Arc::new(OpenAiProvider::new(
                "ollama".into(),
                "http://localhost:11434/v1".into(),
                "ollama".into(),
                "llama3".into(),
            ))),
            tools: Arc::new(builtin_tools(Some(self.memory.clone()))),
            memory: self.memory.clone(),
            policy: PolicyEngine::new(vec![], vec!["~".into()], SecurityLevel::ConfirmExecute),
            system_prompt: default_system_prompt(),
            hooks: HookRegistry::new(),
            skills: Arc::new(SkillRegistry::new()),
        });
        self.agent = Some(AgentLoop::new(harness, session_id));
    }

    /// Send a message with explicit text (from UI text input).
    pub fn send_text(&mut self, text: &str, cx: &mut Context<'_, Self>) {
        let content = text.to_owned();
        if content.trim().is_empty() {
            return;
        }
        self.do_send(content, cx);
    }

    /// Send a message from the internal input_text buffer.
    pub fn send_message(&mut self, cx: &mut Context<'_, Self>) {
        let content = std::mem::take(&mut self.input_text);
        if content.trim().is_empty() {
            return;
        }
        self.do_send(content, cx);
    }

    fn do_send(&mut self, content: String, cx: &mut Context<'_, Self>) {
        self.messages.push(MessageItem {
            role: "user".into(),
            content: content.clone(),
            tool_calls: vec![],
        });
        self.streaming = true;
        cx.notify();

        let mut agent = self.agent.take().expect("agent missing");
        let event_tx = self.event_tx.clone();
        let approval_ch = self.approval_tx.clone();

        cx.spawn(async move |this: WeakEntity<AppModel>, cx: &mut AsyncApp| {
            let _ = agent.run_turn(&content, &event_tx, Some(approval_ch)).await;

            this.update(cx, |this, cx| {
                this.agent = Some(agent);
                this.streaming = false;
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn poll_events(&mut self, cx: &mut Context<'_, Self>) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                AgentEvent::TextDelta { delta, .. } => {
                    if let Some(last) = self.messages.last_mut() {
                        if last.role == "assistant" {
                            last.content.push_str(&delta);
                        } else {
                            self.messages.push(MessageItem {
                                role: "assistant".into(),
                                content: delta,
                                tool_calls: vec![],
                            });
                        }
                    } else {
                        self.messages.push(MessageItem {
                            role: "assistant".into(),
                            content: delta,
                            tool_calls: vec![],
                        });
                    }
                    cx.notify();
                }
                AgentEvent::ToolCallStarted {
                    tool_name, call_id, ..
                } => {
                    if let Some(last) = self.messages.last_mut() {
                        last.tool_calls.push(ToolCallItem {
                            name: tool_name,
                            status: "running".into(),
                            summary: format!("call {call_id}"),
                        });
                    }
                    cx.notify();
                }
                AgentEvent::ToolCallFinished {
                    call_id,
                    tool_name,
                    ok,
                    summary,
                    ..
                } => {
                    if let Some(last) = self.messages.last_mut() {
                        for tc in &mut last.tool_calls {
                            if tc.name == tool_name && tc.status == "running" {
                                tc.status = if ok { "ok".into() } else { "error".into() };
                                tc.summary = summary;
                                break;
                            }
                        }
                    }
                    let _ = call_id;
                    cx.notify();
                }
                AgentEvent::ApprovalRequired {
                    call_id,
                    tool_name,
                    arguments_json,
                    security_level,
                    ..
                } => {
                    self.pending_approval = Some(ApprovalRequest {
                        call_id,
                        tool_name,
                        arguments: arguments_json,
                        security_level,
                    });
                    cx.notify();
                }
                AgentEvent::StreamingDone { .. } => {
                    self.streaming = false;
                    cx.notify();
                }
                AgentEvent::Error { message } => {
                    self.messages.push(MessageItem {
                        role: "system".into(),
                        content: format!("Error: {message}"),
                        tool_calls: vec![],
                    });
                    self.streaming = false;
                    cx.notify();
                }
                _ => {}
            }
        }
    }
}
