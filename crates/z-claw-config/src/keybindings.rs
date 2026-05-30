use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub keys: String,
    pub action: String,
    #[serde(default)]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeybindingConfig {
    #[serde(default)]
    pub bindings: Vec<KeyBinding>,
}

impl KeybindingConfig {
    pub fn defaults() -> Self {
        Self {
            bindings: vec![
                KeyBinding {
                    keys: "enter".into(),
                    action: "send_message".into(),
                    context: Some("chat_input".into()),
                },
                KeyBinding {
                    keys: "escape".into(),
                    action: "close_dialog".into(),
                    context: None,
                },
                KeyBinding {
                    keys: "ctrl+n".into(),
                    action: "new_session".into(),
                    context: None,
                },
                KeyBinding {
                    keys: "ctrl+,".into(),
                    action: "toggle_settings".into(),
                    context: None,
                },
                KeyBinding {
                    keys: "ctrl+p".into(),
                    action: "enter_plan_mode".into(),
                    context: None,
                },
            ],
        }
    }

    pub fn find(&self, keys: &str, context: Option<&str>) -> Option<&str> {
        self.bindings
            .iter()
            .find(|b| b.keys == keys && (b.context.is_none() || b.context.as_deref() == context))
            .map(|b| b.action.as_str())
    }
}
