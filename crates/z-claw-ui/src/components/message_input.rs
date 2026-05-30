use crate::theme::ThemeColors;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct MessageInput {
    pub text: SharedString,
    pub placeholder: SharedString,
}

impl MessageInput {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            placeholder: SharedString::from("Type a message..."),
        }
    }
}

impl RenderOnce for MessageInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let has_text = !self.text.trim().is_empty();
        let display = if self.text.is_empty() {
            self.placeholder
        } else {
            self.text
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .px(px(12.0))
            .py(px(8.0))
            .bg(theme.surface)
            .border_t_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex_1()
                    .p_1()
                    .text_color(if has_text {
                        theme.text
                    } else {
                        theme.text_subtle
                    })
                    .child(display),
            )
            .child(
                div()
                    .px(px(14.0))
                    .py(px(6.0))
                    .bg(if has_text {
                        theme.accent
                    } else {
                        theme.text_subtle
                    })
                    .rounded_md()
                    .text_color(if has_text {
                        theme.background
                    } else {
                        theme.text_muted
                    })
                    .text_sm()
                    .child("Send"),
            )
    }
}
