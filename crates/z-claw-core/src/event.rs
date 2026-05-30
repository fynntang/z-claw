/// Events emitted by the agent loop for UI consumption.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    Ready,

    Error {
        message: String,
    },

    // --- Sessions ---
    SessionCreated {
        id: String,
        title: String,
    },
    SessionsList {
        sessions: Vec<SessionSummary>,
    },

    // --- Messages ---
    /// A text delta during streaming (incremental)
    TextDelta {
        session_id: String,
        delta: String,
    },
    /// Thinking / reasoning content
    ThinkingDelta {
        session_id: String,
        delta: String,
    },
    /// A complete message from assistant
    MessageComplete {
        session_id: String,
        role: String,
        full_text: String,
    },

    // --- Tool Calls ---
    ToolCallStarted {
        session_id: String,
        tool_name: String,
        call_id: String,
    },
    ToolCallArgs {
        session_id: String,
        call_id: String,
        args_delta: String,
    },
    ToolCallFinished {
        session_id: String,
        call_id: String,
        tool_name: String,
        ok: bool,
        summary: String,
    },

    // --- Approval ---
    /// A dangerous tool needs user approval before execution
    ApprovalRequired {
        session_id: String,
        call_id: String,
        tool_name: String,
        arguments_json: String,
        security_level: String,
    },

    // --- Streaming ---
    StreamingStarted {
        session_id: String,
    },
    StreamingDone {
        session_id: String,
    },
}

/// Lightweight session summary for the sidebar list.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: String,
    pub updated_ms: i64,
}
