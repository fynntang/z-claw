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
    pub active_session_id: Option<String>,
    pub on_new_session: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + Send + Sync>>,
    pub on_select_session: Option<Arc<dyn Fn(String, &mut Window, &mut App) + Send + Sync>>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            active_session_id: None,
            on_new_session: None,
            on_select_session: None,
        }
    }

    pub fn with_sessions(mut self, sessions: Vec<SessionSummary>) -> Self {
        self.sessions = sessions;
        self
    }

    pub fn with_active_session(mut self, id: Option<String>) -> Self {
        self.active_session_id = id;
        self
    }

    pub fn on_new_session(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_new_session = Some(Arc::new(handler));
        self
    }

    pub fn on_select_session(
        mut self,
        handler: impl Fn(String, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_select_session = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .h_full()
            .bg(theme.sidebar_bg)
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(18.0))
                    .pt(px(18.0))
                    .pb(px(12.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .size(px(24.0))
                                    .rounded_md()
                                    .bg(theme.accent)
                                    .border_1()
                                    .border_color(theme.accent),
                            )
                            .child(
                                Label::new("z-claw")
                                    .color(theme.text)
                                    .size(LabelSize::Lg)
                                    .weight(FontWeight::SEMIBOLD),
                            ),
                    )
                    .child(
                        div()
                            .px(px(7.0))
                            .py(px(3.0))
                            .rounded_md()
                            .bg(theme.surface)
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .child("local"),
                    ),
            )
            .child(
                div()
                    .px(px(18.0))
                    .pb(px(14.0))
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child("Agent workspace"),
            )
            .child(
                div().px(px(14.0)).pb(px(14.0)).child(
                    Button::new("+ New Session")
                        .variant(ButtonVariant::Primary)
                        .on_click({
                            let handler = self.on_new_session.clone();
                            move |event, window, cx| {
                                if let Some(ref h) = handler {
                                    h(event, window, cx);
                                }
                                cx.stop_propagation();
                            }
                        }),
                ),
            )
            .child(
                div()
                    .px(px(18.0))
                    .pb(px(8.0))
                    .text_xs()
                    .text_color(theme.text_subtle)
                    .font_weight(FontWeight::MEDIUM)
                    .child("Sessions"),
            )
            .child(
                div()
                    .flex_1()
                    .px(px(10.0))
                    .children(self.sessions.iter().map(|s| {
                        let session_id = s.id.clone();
                        let h = self.on_select_session.clone();
                        let is_active = self.active_session_id.as_ref() == Some(&s.id);
                        let short_id = s.id.chars().take(8).collect::<String>();
                        div()
                            .px(px(10.0))
                            .py(px(10.0))
                            .mb(px(3.0))
                            .rounded_md()
                            .border_1()
                            .border_color(if is_active {
                                theme.border
                            } else {
                                theme.sidebar_bg
                            })
                            .bg(if is_active {
                                theme.surface
                            } else {
                                theme.sidebar_bg
                            })
                            .text_color(if is_active {
                                theme.text
                            } else {
                                theme.text_muted
                            })
                            .text_sm()
                            .cursor_pointer()
                            .hover(|el| el.bg(theme.surface))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(div().w(px(3.0)).h(px(18.0)).rounded_md().bg(
                                        if is_active {
                                            theme.accent
                                        } else {
                                            theme.border
                                        },
                                    ))
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                Label::new(s.title.clone())
                                                    .color(if is_active {
                                                        theme.text
                                                    } else {
                                                        theme.text_muted
                                                    })
                                                    .size(LabelSize::Sm)
                                                    .weight(FontWeight::MEDIUM),
                                            )
                                            .child(
                                                Label::new(short_id)
                                                    .color(theme.text_subtle)
                                                    .size(LabelSize::Xs),
                                            ),
                                    ),
                            )
                            .on_mouse_down(MouseButton::Left, {
                                let id = session_id.clone();
                                move |_, window, cx| {
                                    if let Some(ref h) = h {
                                        h(id.clone(), window, cx);
                                    }
                                    cx.stop_propagation();
                                }
                            })
                    }))
                    .when(self.sessions.is_empty(), |el| {
                        el.child(
                            div()
                                .px(px(8.0))
                                .py(px(10.0))
                                .text_sm()
                                .text_color(theme.text_subtle)
                                .child("No sessions yet"),
                        )
                    }),
            )
            .child(
                div()
                    .px(px(18.0))
                    .py(px(14.0))
                    .border_t_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(div().size(px(7.0)).rounded_full().bg(theme.success))
                            .child(
                                Label::new("Runtime ready")
                                    .color(theme.text_subtle)
                                    .size(LabelSize::Xs),
                            ),
                    ),
            )
    }
}
