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
        let rail_color = if is_user { theme.accent } else { theme.success };
        let role_label = if is_user { "You" } else { "Assistant" };
        let has_tools = !self.message.tool_calls.is_empty();

        let base = div().flex().flex_row();

        base.child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .gap(px(12.0))
                .child(
                    div()
                        .mt(px(5.0))
                        .w(px(3.0))
                        .h(px(28.0))
                        .rounded_md()
                        .bg(rail_color),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .max_w(px(720.0))
                        .child(
                            div()
                                .mb(px(5.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    Label::new(role_label)
                                        .color(theme.text_muted)
                                        .size(LabelSize::Xs)
                                        .weight(FontWeight::MEDIUM),
                                )
                                .child(div().w(px(18.0)).h(px(1.0)).bg(theme.border)),
                        )
                        .child({
                            let mut bubble = div()
                                .bg(bubble_bg)
                                .rounded_md()
                                .border_1()
                                .border_color(theme.border)
                                .px(px(14.0))
                                .py(px(11.0))
                                .text_color(theme.text)
                                .text_sm()
                                .child(render_markdown(&self.message.content, cx));

                            if has_tools {
                                bubble =
                                    bubble.children(self.message.tool_calls.iter().map(|tc| {
                                        let status_color = match tc.status.as_str() {
                                            "ok" => theme.success,
                                            "error" => theme.error,
                                            _ => theme.warning,
                                        };
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap_2()
                                            .mt(px(8.0))
                                            .pt(px(8.0))
                                            .border_t_1()
                                            .border_color(theme.border)
                                            .child(
                                                Label::new(format!("[{}]", tc.status))
                                                    .color(status_color)
                                                    .size(LabelSize::Xs),
                                            )
                                            .child(
                                                Label::new(format!("{} - {}", tc.name, tc.summary))
                                                    .color(theme.text_muted)
                                                    .size(LabelSize::Xs),
                                            )
                                    }));
                            }

                            bubble
                        }),
                ),
        )
    }
}
