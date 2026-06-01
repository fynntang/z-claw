use crate::components::primitives::{Button, ButtonVariant};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;
use std::sync::Arc;

/// Chat input bar with text display and send button.
#[derive(Clone, IntoElement)]
pub struct ChatInputBar {
    pub text: SharedString,
    pub placeholder: SharedString,
    pub disabled: bool,
    pub on_send: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync>>,
}

impl ChatInputBar {
    pub fn new() -> Self {
        Self {
            text: SharedString::default(),
            placeholder: SharedString::from("Type a message..."),
            disabled: false,
            on_send: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_send(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_send = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for ChatInputBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let has_text = !self.text.trim().is_empty();
        let can_send = has_text && !self.disabled;
        let display = if self.text.is_empty() {
            self.placeholder.clone()
        } else {
            self.text.clone()
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .px(px(16.0))
            .py(px(10.0))
            .bg(theme.surface)
            .border_t_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex_1()
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(theme.background)
                    .rounded_md()
                    .text_sm()
                    .text_color(if has_text {
                        theme.text
                    } else {
                        theme.text_subtle
                    })
                    .child(display),
            )
            .child(
                div()
                    .ml(px(8.0))
                    .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                        if can_send {
                            if let Some(ref handler) = self.on_send {
                                handler(event, window, cx);
                            }
                        }
                    })
                    .child(
                        Button::new("Send")
                            .variant(if can_send {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Secondary
                            })
                            .disabled(!can_send),
                    ),
            )
    }
}
