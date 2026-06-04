use crate::app::{MessageItem, ToolCallItem};
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
        let role_label = if is_user { "You" } else { "z-claw" };
        let avatar_label = if is_user { "U" } else { "Z" };
        let avatar_bg = if is_user {
            theme.accent
        } else {
            theme.accent_subtle
        };

        div()
            .flex()
            .flex_row()
            .gap(px(12.0))
            .child(
                div()
                    .mt(px(2.0))
                    .size(px(26.0))
                    .min_w(px(26.0))
                    .rounded_sm()
                    .bg(avatar_bg)
                    .border_1()
                    .border_color(if is_user {
                        theme.accent
                    } else {
                        theme.border_strong
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if is_user {
                        theme.background
                    } else {
                        theme.accent_text
                    })
                    .child(avatar_label),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .flex_col()
                    .child(message_header(role_label, is_user, theme))
                    .child(
                        div()
                            .text_sm()
                            .line_height(px(22.0))
                            .text_color(theme.text_muted)
                            .child(render_markdown(&self.message.content, cx)),
                    )
                    .children(
                        self.message
                            .tool_calls
                            .iter()
                            .map(|tool_call| tool_block(tool_call, theme)),
                    ),
            )
    }
}

fn message_header(
    role_label: &'static str,
    is_user: bool,
    theme: &ThemeColors,
) -> impl IntoElement {
    div()
        .mb(px(3.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .child(
            Label::new(role_label)
                .color(theme.text)
                .size(LabelSize::Sm)
                .weight(FontWeight::BOLD),
        )
        .child(
            div()
                .px(px(5.0))
                .py(px(1.0))
                .rounded_sm()
                .border_1()
                .border_color(theme.border)
                .bg(theme.sidebar_bg)
                .text_size(px(9.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_subtle)
                .child(if is_user { "request" } else { "response" }),
        )
        .when(!is_user, |el| {
            el.child(
                div()
                    .px(px(5.0))
                    .py(px(1.0))
                    .rounded_sm()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.sidebar_bg)
                    .text_size(px(9.0))
                    .text_color(theme.text_subtle)
                    .child("local/llama3"),
            )
        })
}

fn tool_block(tool_call: &ToolCallItem, theme: &ThemeColors) -> impl IntoElement {
    let (status_bg, status_fg, status_label) = match tool_call.status.as_str() {
        "ok" | "success" => (theme.success_bg, theme.success, "success"),
        "error" => (theme.error_bg, theme.error, "error"),
        "running" => (theme.info_bg, theme.info, "running"),
        _ => (theme.warning_bg, theme.warning, "pending"),
    };

    div()
        .mt(px(9.0))
        .rounded_sm()
        .border_1()
        .border_color(theme.border)
        .bg(theme.sidebar_bg)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .px(px(12.0))
                .py(px(7.0))
                .border_b_1()
                .border_color(theme.border)
                .bg(theme.surface)
                .child(
                    div()
                        .px(px(5.0))
                        .py(px(1.0))
                        .rounded_sm()
                        .bg(theme.accent_subtle)
                        .text_size(px(9.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.accent_text)
                        .child("tool"),
                )
                .child(
                    Label::new(tool_call.name.clone())
                        .color(theme.text)
                        .size(LabelSize::Xs)
                        .weight(FontWeight::BOLD),
                )
                .child(
                    div()
                        .flex_1()
                        .text_size(px(10.0))
                        .text_color(theme.text_subtle)
                        .child(tool_call.summary.clone()),
                )
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(1.0))
                        .rounded_full()
                        .bg(status_bg)
                        .text_size(px(9.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(status_fg)
                        .child(status_label),
                ),
        )
}
