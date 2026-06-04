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
            let mut area = div().flex_1().bg(theme.background).child(
                div()
                    .mx_auto()
                    .w_full()
                    .max_w(px(860.0))
                    .px(px(28.0))
                    .py(px(22.0))
                    .children(
                        self.messages
                            .into_iter()
                            .map(|msg| div().mb(px(18.0)).child(MessageBubble { message: msg })),
                    ),
            );
            area.style().overflow.y = Some(Overflow::Scroll);
            area
        } else {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .px(px(40.0))
                .bg(theme.background)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .max_w(px(560.0))
                        .child(
                            div()
                                .mb(px(12.0))
                                .size(px(42.0))
                                .rounded_md()
                                .bg(theme.accent_subtle)
                                .border_1()
                                .border_color(theme.border_strong)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(theme.accent_text)
                                .font_weight(FontWeight::BOLD)
                                .child("Z"),
                        )
                        .child(
                            Label::new("Ready")
                                .color(theme.text)
                                .size(LabelSize::Lg)
                                .weight(FontWeight::SEMIBOLD),
                        )
                        .child(
                            div().mt(px(6.0)).mb(px(20.0)).child(
                                Label::new("Start with a task, inspect the workspace, or ask the agent to run a focused workflow.")
                                    .color(theme.text_muted)
                                    .size(LabelSize::Sm),
                            ),
                        )
                        .child(
                            div()
                                .mb(px(8.0))
                                .w_full()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_subtle)
                                .child("Suggested starting points"),
                        )
                        .child(
                            div()
                                .grid()
                                .grid_cols(2)
                                .gap(px(8.0))
                                .w_full()
                                .child(empty_card("Build", "Implement a feature or fix a failing check", theme))
                                .child(empty_card("Analyze", "Review code, trace errors, or inspect architecture", theme))
                                .child(empty_card("Automate", "Prepare commits, tests, approvals, and PR work", theme))
                                .child(empty_card("Research", "Look up docs, APIs, and implementation patterns", theme)),
                        ),
                )
        }
    }
}

fn empty_card(
    title: &'static str,
    description: &'static str,
    theme: &ThemeColors,
) -> impl IntoElement {
    div()
        .px(px(14.0))
        .py(px(12.0))
        .rounded_sm()
        .border_1()
        .border_color(theme.border)
        .bg(theme.sidebar_bg)
        .child(
            Label::new(title)
                .color(theme.text)
                .size(LabelSize::Sm)
                .weight(FontWeight::SEMIBOLD),
        )
        .child(
            div().mt(px(2.0)).child(
                Label::new(description)
                    .color(theme.text_muted)
                    .size(LabelSize::Xs),
            ),
        )
}
