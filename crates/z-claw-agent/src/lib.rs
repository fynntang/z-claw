mod hooks;
mod scheduler;

pub use hooks::{Hook, HookEvent, HookRegistry};
pub use scheduler::{CronScheduler, CronTask};

use std::sync::Arc;
use tokio::sync::mpsc;
use z_claw_core::{AgentEvent, ChatMessage, GenerateConfig, StreamChunk, ToolCall};
use z_claw_memory::MemoryBackend;
use z_claw_providers::ProviderChain;
use z_claw_security::PolicyEngine;
use z_claw_skills::SkillRegistry;
use z_claw_tools::ToolRegistry;

const MAX_ROUNDS: usize = 10;

/// Combines providers, tools, and memory into one harness for the agent loop.
pub struct Harness {
    pub providers: ProviderChain,
    pub tools: Arc<ToolRegistry>,
    pub memory: Arc<dyn MemoryBackend>,
    pub policy: PolicyEngine,
    pub system_prompt: String,
    pub hooks: HookRegistry,
    pub skills: Arc<SkillRegistry>,
}

/// The agent loop — runs turns with streaming and tool calling.
pub struct AgentLoop {
    harness: Arc<Harness>,
    session_id: String,
    history: Vec<ChatMessage>,
    session_created: bool,
    plan_mode: bool,
}

impl AgentLoop {
    pub fn new(harness: Arc<Harness>, session_id: String) -> Self {
        Self {
            harness,
            session_id,
            history: Vec::new(),
            session_created: false,
            plan_mode: false,
        }
    }

    /// Enter plan mode — restricts to read-only tools.
    pub fn enter_plan_mode(&mut self) {
        self.plan_mode = true;
    }

    /// Exit plan mode — restore full tool access.
    pub fn exit_plan_mode(&mut self) {
        self.plan_mode = false;
    }

    pub fn is_plan_mode(&self) -> bool {
        self.plan_mode
    }

    /// Run one user turn, emitting events for the UI.
    pub async fn run_turn(
        &mut self,
        user_input: &str,
        event_tx: &mpsc::UnboundedSender<AgentEvent>,
    ) -> Result<String, z_claw_core::ClawError> {
        // Ensure session exists in persistent storage
        if !self.session_created {
            self.harness
                .memory
                .create_session(&self.session_id, "New Session")
                .await
                .ok();
            self.session_created = true;
        }

        // Add user message
        self.history.push(ChatMessage {
            role: "user".into(),
            content: user_input.into(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });
        self.harness
            .memory
            .append_message(&self.session_id, "user", user_input, None)
            .await
            .ok();

        let _ = event_tx.send(AgentEvent::StreamingStarted {
            session_id: self.session_id.clone(),
        });

        // Inner loop: model turns with tool calling
        for _round in 0..MAX_ROUNDS {
            // Build system prompt with skills and plan mode context
            let mut system = self.harness.system_prompt.clone();

            // Inject active skills
            let skills_prompt = self.harness.skills.active_skills_prompt(None);
            if !skills_prompt.is_empty() {
                system.push_str("\n\n");
                system.push_str(&skills_prompt);
            }

            // Plan mode context
            if self.plan_mode {
                system.push_str("\n\nYou are in PLAN MODE. Only use read-only tools (read_file, list_directory, search_memory). Do NOT modify files or execute commands. Propose a plan first, then wait for user approval.");
            }

            let mut messages = vec![ChatMessage {
                role: "system".into(),
                content: system,
                tool_calls: None,
                tool_call_id: None,
                name: None,
            }];
            messages.extend(self.history.clone());

            let tool_defs = self.harness.tools.definitions();

            let config = GenerateConfig {
                model: String::new(), // use default from provider
                temperature: None,
                max_tokens: None,
                stream: true,
            };

            // Stream model response
            let mut stream = self
                .harness
                .providers
                .chat(messages, tool_defs, &config)
                .await?;

            let mut text = String::new();
            let mut tool_calls: Vec<ToolCallBuilder> = Vec::new();

            use futures::StreamExt;
            while let Some(chunk) = stream.next().await {
                match chunk? {
                    StreamChunk::TextDelta(delta) => {
                        text.push_str(&delta);
                        let _ = event_tx.send(AgentEvent::TextDelta {
                            session_id: self.session_id.clone(),
                            delta,
                        });
                    }
                    StreamChunk::ToolCallStart { index, id, name } => {
                        // Ensure vec is large enough
                        while tool_calls.len() <= index {
                            tool_calls.push(ToolCallBuilder::default());
                        }
                        tool_calls[index].id = id.clone();
                        tool_calls[index].name = name.clone();
                        let _ = event_tx.send(AgentEvent::ToolCallStarted {
                            session_id: self.session_id.clone(),
                            tool_name: name,
                            call_id: id,
                        });
                    }
                    StreamChunk::ToolCallDelta { index, args_delta } => {
                        if index < tool_calls.len() {
                            tool_calls[index].args.push_str(&args_delta);
                            let _ = event_tx.send(AgentEvent::ToolCallArgs {
                                session_id: self.session_id.clone(),
                                call_id: tool_calls[index].id.clone(),
                                args_delta,
                            });
                        }
                    }
                    StreamChunk::Done { .. } => break,
                    _ => {}
                }
            }

            // If no tool calls, we are done
            let finished_calls: Vec<ToolCall> = tool_calls
                .into_iter()
                .filter(|tc| !tc.name.is_empty())
                .map(|tc| ToolCall {
                    id: tc.id,
                    name: tc.name,
                    arguments: serde_json::from_str(&tc.args).unwrap_or(serde_json::Value::Null),
                })
                .collect();

            if finished_calls.is_empty() {
                // Add assistant message and finish
                self.history.push(ChatMessage {
                    role: "assistant".into(),
                    content: text.clone(),
                    tool_calls: None,
                    tool_call_id: None,
                    name: None,
                });
                self.harness
                    .memory
                    .append_message(&self.session_id, "assistant", &text, None)
                    .await
                    .ok();
                let _ = event_tx.send(AgentEvent::MessageComplete {
                    session_id: self.session_id.clone(),
                    role: "assistant".into(),
                    full_text: text.clone(),
                });
                let _ = event_tx.send(AgentEvent::StreamingDone {
                    session_id: self.session_id.clone(),
                });
                return Ok(text);
            }

            // Add assistant message with tool calls
            let tool_call_json = serde_json::to_value(&finished_calls).ok();
            self.history.push(ChatMessage {
                role: "assistant".into(),
                content: text.clone(),
                tool_calls: tool_call_json.clone(),
                tool_call_id: None,
                name: None,
            });
            self.harness
                .memory
                .append_message(&self.session_id, "assistant", &text, tool_call_json.clone())
                .await
                .ok();

            // Execute each tool call
            for tc in &finished_calls {
                // Security check
                let level = self.harness.policy.classify(&tc.name, &tc.arguments);
                if self.harness.policy.is_blocked(level) {
                    let _ = event_tx.send(AgentEvent::ToolCallFinished {
                        session_id: self.session_id.clone(),
                        call_id: tc.id.clone(),
                        tool_name: tc.name.clone(),
                        ok: false,
                        summary: format!("Blocked: security level {level}"),
                    });
                    let blocked_content = format!("Blocked: security level {}", level);
                    self.history.push(ChatMessage {
                        role: "tool".into(),
                        content: blocked_content.clone(),
                        tool_calls: None,
                        tool_call_id: Some(tc.id.clone()),
                        name: Some(tc.name.clone()),
                    });
                    self.harness
                        .memory
                        .append_message(&self.session_id, "tool", &blocked_content, None)
                        .await
                        .ok();
                    continue;
                }

                if self.harness.policy.needs_approval(level) {
                    let _ = event_tx.send(AgentEvent::ApprovalRequired {
                        session_id: self.session_id.clone(),
                        call_id: tc.id.clone(),
                        tool_name: tc.name.clone(),
                        arguments_json: serde_json::to_string(&tc.arguments).unwrap_or_default(),
                        security_level: level.to_string(),
                    });
                    // In MVP without UI approval hooks, auto-deny for RequireApproval+ level
                    // Phase 2 will add proper async approval
                    let _ = event_tx.send(AgentEvent::ToolCallFinished {
                        session_id: self.session_id.clone(),
                        call_id: tc.id.clone(),
                        tool_name: tc.name.clone(),
                        ok: false,
                        summary: "Approval required — auto-denied in MVP".into(),
                    });
                    let denied_content = "Approval required — auto-denied in MVP";
                    self.history.push(ChatMessage {
                        role: "tool".into(),
                        content: denied_content.into(),
                        tool_calls: None,
                        tool_call_id: Some(tc.id.clone()),
                        name: Some(tc.name.clone()),
                    });
                    self.harness
                        .memory
                        .append_message(&self.session_id, "tool", denied_content, None)
                        .await
                        .ok();
                    continue;
                }

                // Run PreToolUse hooks
                self.harness
                    .hooks
                    .run_hooks(&HookEvent::PreToolUse, Some(&tc.name));

                // Execute the tool
                match self.harness.tools.get(&tc.name) {
                    Some(tool) => match tool.execute(tc.arguments.clone()).await {
                        Ok(result) => {
                            // Run PostToolUse hooks
                            self.harness
                                .hooks
                                .run_hooks(&HookEvent::PostToolUse, Some(&tc.name));

                            let _ = event_tx.send(AgentEvent::ToolCallFinished {
                                session_id: self.session_id.clone(),
                                call_id: tc.id.clone(),
                                tool_name: tc.name.clone(),
                                ok: true,
                                summary: result.clone(),
                            });
                            self.history.push(ChatMessage {
                                role: "tool".into(),
                                content: result.clone(),
                                tool_calls: None,
                                tool_call_id: Some(tc.id.clone()),
                                name: Some(tc.name.clone()),
                            });
                            self.harness
                                .memory
                                .append_message(&self.session_id, "tool", &result, None)
                                .await
                                .ok();
                        }
                        Err(e) => {
                            let _ = event_tx.send(AgentEvent::ToolCallFinished {
                                session_id: self.session_id.clone(),
                                call_id: tc.id.clone(),
                                tool_name: tc.name.clone(),
                                ok: false,
                                summary: e.to_string(),
                            });
                            let error_content = format!("Error: {e}");
                            self.history.push(ChatMessage {
                                role: "tool".into(),
                                content: error_content.clone(),
                                tool_calls: None,
                                tool_call_id: Some(tc.id.clone()),
                                name: Some(tc.name.clone()),
                            });
                            self.harness
                                .memory
                                .append_message(&self.session_id, "tool", &error_content, None)
                                .await
                                .ok();
                        }
                    },
                    None => {
                        let _ = event_tx.send(AgentEvent::ToolCallFinished {
                            session_id: self.session_id.clone(),
                            call_id: tc.id.clone(),
                            tool_name: tc.name.clone(),
                            ok: false,
                            summary: format!("Tool not found: {}", tc.name),
                        });
                        let not_found_content = format!("Tool not found: {}", tc.name);
                        self.history.push(ChatMessage {
                            role: "tool".into(),
                            content: not_found_content.clone(),
                            tool_calls: None,
                            tool_call_id: Some(tc.id.clone()),
                            name: Some(tc.name.clone()),
                        });
                        self.harness
                            .memory
                            .append_message(&self.session_id, "tool", &not_found_content, None)
                            .await
                            .ok();
                    }
                }
            }
            // Continue to next round — model sees tool results
        }

        let msg = format!("Agent loop exceeded max rounds ({MAX_ROUNDS})");
        let _ = event_tx.send(AgentEvent::Error {
            message: msg.clone(),
        });
        let _ = event_tx.send(AgentEvent::StreamingDone {
            session_id: self.session_id.clone(),
        });
        Err(z_claw_core::ClawError::MaxRoundsExceeded(MAX_ROUNDS))
    }
}

#[derive(Default)]
struct ToolCallBuilder {
    id: String,
    name: String,
    args: String,
}

/// Build the default system prompt.
pub fn default_system_prompt() -> String {
    r#"You are a helpful AI assistant running as a desktop application.
You have access to tools for reading/writing files and executing commands.
When you need to perform an action, use the appropriate tool.
Always explain what you are doing before using a tool.
Keep responses concise and helpful."#
        .to_string()
}
