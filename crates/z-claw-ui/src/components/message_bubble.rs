use crate::app::MessageItem;
use crate::components::markdown::render_markdown;
use crate::components::{Label, LabelSize};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct MessageBubble {
    pub message: MessageItem,
}

impl RenderOnce for MessageBubble {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let is_user = self.message.role == "user";
        let bubble_bg = if is_user {
            theme.overlay
        } else {
            theme.surface
        };
        let role_label = if is_user { "You" } else { "Assistant" };
        let has_tools = !self.message.tool_calls.is_empty();

        let mut base = div().flex().flex_row().mb_2();

        if is_user {
            base = base.justify_end();
        } else {
            base = base.justify_start();
        }

        base.child(
            div()
                .flex()
                .flex_col()
                .max_w(px(560.0))
                .child(
                    div().mb_1().child(
                        Label::new(role_label)
                            .color(theme.text_muted)
                            .size(LabelSize::Xs),
                    ),
                )
                .child({
                    let mut bubble = div()
                        .bg(bubble_bg)
                        .rounded_lg()
                        .px(px(14.0))
                        .py(px(10.0))
                        .text_color(theme.text)
                        .text_sm()
                        .child(render_markdown(&self.message.content, theme));

                    if has_tools {
                        bubble = bubble.children(self.message.tool_calls.iter().map(|tc| {
                            let status_color = match tc.status.as_str() {
                                "ok" => theme.success,
                                "error" => theme.error,
                                _ => theme.warning,
                            };
                            div()
                                .flex()
                                .flex_row()
                                .gap_2()
                                .mt_1()
                                .pt_1()
                                .border_t_1()
                                .border_color(theme.border)
                                .child(
                                    Label::new(format!("[{}]", tc.status))
                                        .color(status_color)
                                        .size(LabelSize::Xs),
                                )
                                .child(
                                    Label::new(format!("{} — {}", tc.name, tc.summary))
                                        .color(theme.text_muted)
                                        .size(LabelSize::Xs),
                                )
                        }));
                    }

                    bubble
                }),
        )
    }
}
