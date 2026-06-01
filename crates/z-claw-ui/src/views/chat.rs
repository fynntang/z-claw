use std::sync::Arc;

use crate::components::{ChatInputBar, Label, LabelSize, MessageBubble};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct ChatView {
    pub messages: Vec<crate::app::MessageItem>,
    pub streaming: bool,
    pub input_text: SharedString,
    pub on_send: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync>>,
}

impl RenderOnce for ChatView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let has_messages = !self.messages.is_empty();

        let message_area = if has_messages {
            div().flex_1().p(px(16.0)).children(
                self.messages
                    .into_iter()
                    .map(|msg| div().child(MessageBubble { message: msg })),
            )
        } else {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .child(
                    Label::new("z-claw")
                        .color(theme.text_muted)
                        .size(LabelSize::Lg),
                )
                .child(
                    div().mt(px(4.0)).child(
                        Label::new("Start a conversation. Type a message and press Send.")
                            .color(theme.text_subtle)
                            .size(LabelSize::Sm),
                    ),
                )
        };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(message_area)
            .child(
                ChatInputBar::new()
                    .with_text(self.input_text)
                    .with_disabled(self.streaming)
                    .on_send({
                        let handler = self.on_send.clone();
                        move |event, window, cx| {
                            if let Some(ref h) = handler {
                                h(event, window, cx);
                            }
                        }
                    }),
            )
    }
}
