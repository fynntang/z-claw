use crate::components::{Label, LabelSize, MessageBubble};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct ChatView {
    pub messages: Vec<crate::app::MessageItem>,
}

impl RenderOnce for ChatView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let has_messages = !self.messages.is_empty();

        if has_messages {
            let mut area = div()
                .flex_1()
                .px(px(28.0))
                .py(px(22.0))
                .bg(theme.background);
            area.style().overflow.y = Some(Overflow::Scroll);
            area.children(
                self.messages
                    .into_iter()
                    .map(|msg| div().mb(px(12.0)).child(MessageBubble { message: msg })),
            )
        } else {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .justify_center()
                .px(px(64.0))
                .bg(theme.background)
                .child(
                    Label::new("Ready")
                        .color(theme.text)
                        .size(LabelSize::Lg)
                        .weight(FontWeight::SEMIBOLD),
                )
                .child(
                    div().mt(px(8.0)).max_w(px(420.0)).child(
                        Label::new("No messages in this session.")
                            .color(theme.text_subtle)
                            .size(LabelSize::Sm),
                    ),
                )
        }
    }
}
