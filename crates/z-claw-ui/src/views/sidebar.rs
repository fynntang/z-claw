use std::sync::Arc;

use crate::components::{Button, ButtonVariant, Label, LabelSize};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub id: String,
    pub title: String,
    pub updated_ms: i64,
}

#[derive(IntoElement)]
pub struct Sidebar {
    pub sessions: Vec<SessionSummary>,
    pub on_new_session: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync>>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            on_new_session: None,
        }
    }

    pub fn with_sessions(mut self, sessions: Vec<SessionSummary>) -> Self {
        self.sessions = sessions;
        self
    }

    pub fn on_new_session(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_new_session = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        div()
            .flex()
            .flex_col()
            .w(px(240.0))
            .h_full()
            .bg(theme.sidebar_bg)
            .border_r_1()
            .border_color(theme.border)
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(14.0))
                    .pt(px(14.0))
                    .pb(px(10.0))
                    .child(
                        Label::new("z-claw")
                            .color(theme.text.into())
                            .size(LabelSize::Lg),
                    ),
            )
            .child(
                // New session button
                div()
                    .mx(px(10.0))
                    .mb(px(10.0))
                    .on_mouse_down(MouseButton::Left, {
                        let handler = self.on_new_session.clone();
                        move |event, window, cx| {
                            if let Some(ref h) = handler {
                                h(event, window, cx);
                            }
                        }
                    })
                    .child(Button::new("+ New Session").variant(ButtonVariant::Primary)),
            )
            .child(
                // Session list
                div().flex_1().children(self.sessions.iter().map(|s| {
                    div()
                        .px(px(12.0))
                        .py(px(8.0))
                        .mx(px(8.0))
                        .mb(px(2.0))
                        .rounded_md()
                        .text_color(theme.text_muted)
                        .text_sm()
                        .child(s.title.clone())
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(s.id[..8].to_string()),
                        )
                })),
            )
    }
}
