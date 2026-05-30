use crate::{AgentLoop, Harness};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Spawn a sub-agent for an isolated task. Returns (id, rx_for_result).
pub async fn spawn_sub_agent(
    harness: Arc<Harness>,
    task: &str,
    parent_session: &str,
) -> (String, mpsc::UnboundedReceiver<String>) {
    let id = uuid::Uuid::new_v4().to_string();
    let (tx, rx) = mpsc::unbounded_channel();
    let session_id = format!("{parent_session}/sub/{id}");
    let task_owned = task.to_string();

    tokio::spawn(async move {
        let mut agent = AgentLoop::new(harness, session_id);
        let (event_tx, _event_rx) = mpsc::unbounded_channel::<z_claw_core::AgentEvent>();
        match agent.run_turn(&task_owned, &event_tx, None).await {
            Ok(result) => {
                let _ = tx.send(result);
            }
            Err(e) => {
                let _ = tx.send(format!("Sub-agent error: {e}"));
            }
        }
    });

    (id, rx)
}
