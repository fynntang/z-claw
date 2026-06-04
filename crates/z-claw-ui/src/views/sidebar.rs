use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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
        let session_count = self.sessions.len();
        let active_index = self
            .sessions
            .iter()
            .position(|session| self.active_session_id.as_ref() == Some(&session.id))
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .w(px(260.0))
            .min_w(px(260.0))
            .h_full()
            .bg(theme.sidebar_bg)
            .border_r_1()
            .border_color(theme.border)
            .child(sidebar_header(theme, self.on_new_session.clone()))
            .child(section_label("Sessions", theme))
            .child(
                div()
                    .flex_1()
                    .px(px(8.0))
                    .py(px(2.0))
                    .children(self.sessions.iter().enumerate().map(|(index, session)| {
                        let session_id = session.id.clone();
                        let handler = self.on_select_session.clone();
                        let is_active = self.active_session_id.as_ref() == Some(&session.id);
                        let status = if is_active {
                            SessionStatus::Running
                        } else if index % 4 == 0 {
                            SessionStatus::Done
                        } else {
                            SessionStatus::Idle
                        };

                        session_row(session, status, is_active, theme).on_mouse_down(
                            MouseButton::Left,
                            {
                                let id = session_id.clone();
                                move |_, window, cx| {
                                    if let Some(ref handler) = handler {
                                        handler(id.clone(), window, cx);
                                    }
                                    cx.stop_propagation();
                                }
                            },
                        )
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
            .child(sidebar_stats(session_count, active_index, theme))
            .child(sidebar_footer(theme))
    }
}

fn sidebar_header(
    theme: &ThemeColors,
    on_new_session: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + Send + Sync>>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .px(px(12.0))
        .py(px(10.0))
        .border_b_1()
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .size(px(22.0))
                        .rounded_md()
                        .bg(theme.accent)
                        .border_1()
                        .border_color(theme.accent_hover),
                )
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(
                            Label::new("z-claw")
                                .color(theme.text)
                                .size(LabelSize::Lg)
                                .weight(FontWeight::SEMIBOLD),
                        )
                        .child(
                            Label::new("Agent workspace")
                                .color(theme.text_muted)
                                .size(LabelSize::Xs),
                        ),
                )
                .child(
                    div()
                        .px(px(7.0))
                        .py(px(3.0))
                        .rounded_sm()
                        .bg(theme.surface)
                        .border_1()
                        .border_color(theme.border)
                        .text_size(px(10.0))
                        .text_color(theme.text_subtle)
                        .child("local"),
                ),
        )
        .child(
            Button::new("+ New Session")
                .variant(ButtonVariant::Secondary)
                .on_click({
                    let handler = on_new_session.clone();
                    move |event, window, cx| {
                        if let Some(ref handler) = handler {
                            handler(event, window, cx);
                        }
                        cx.stop_propagation();
                    }
                }),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(4.0))
                .child(
                    div()
                        .flex_1()
                        .h(px(26.0))
                        .flex()
                        .items_center()
                        .px(px(8.0))
                        .rounded_sm()
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.background)
                        .text_size(px(11.0))
                        .text_color(theme.text_subtle)
                        .child("Search sessions"),
                )
                .child(filter_badge("all", true, theme))
                .child(filter_badge("run", false, theme)),
        )
}

fn filter_badge(label: &'static str, active: bool, theme: &ThemeColors) -> impl IntoElement {
    div()
        .h(px(26.0))
        .px(px(8.0))
        .flex()
        .items_center()
        .rounded_sm()
        .border_1()
        .border_color(if active { theme.accent } else { theme.border })
        .bg(if active {
            theme.accent_subtle
        } else {
            theme.sidebar_bg
        })
        .text_size(px(10.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(if active {
            theme.accent_text
        } else {
            theme.text_muted
        })
        .child(label)
}

fn section_label(label: &'static str, theme: &ThemeColors) -> impl IntoElement {
    div()
        .px(px(12.0))
        .pt(px(10.0))
        .pb(px(4.0))
        .text_size(px(10.0))
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_subtle)
        .child(label)
}

#[derive(Clone, Copy)]
enum SessionStatus {
    Running,
    Done,
    Idle,
}

fn session_row(
    session: &SessionSummary,
    status: SessionStatus,
    is_active: bool,
    theme: &ThemeColors,
) -> Div {
    let short_id = session.id.chars().take(8).collect::<String>();
    let status_color = match status {
        SessionStatus::Running => theme.accent,
        SessionStatus::Done => theme.success,
        SessionStatus::Idle => theme.text_subtle,
    };
    let status_label = match status {
        SessionStatus::Running => "wip",
        SessionStatus::Done => "done",
        SessionStatus::Idle => "idle",
    };

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .px(px(10.0))
        .py(px(8.0))
        .mb(px(2.0))
        .rounded_sm()
        .cursor_pointer()
        .bg(if is_active {
            theme.accent_subtle
        } else {
            theme.sidebar_bg
        })
        .border_1()
        .border_color(if is_active {
            theme.border_strong
        } else {
            theme.sidebar_bg
        })
        .hover(|el| el.bg(theme.overlay).border_color(theme.border))
        .child(div().size(px(6.0)).rounded_full().bg(status_color))
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .flex()
                .flex_col()
                .child(
                    Label::new(session.title.clone())
                        .color(if is_active {
                            theme.text
                        } else {
                            theme.text_muted
                        })
                        .size(LabelSize::Sm)
                        .weight(FontWeight::SEMIBOLD),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(6.0))
                        .child(
                            Label::new(short_id)
                                .color(theme.text_subtle)
                                .size(LabelSize::Xs),
                        )
                        .child(
                            Label::new(session_age(session.updated_ms))
                                .color(theme.text_subtle)
                                .size(LabelSize::Xs),
                        ),
                ),
        )
        .child(
            div()
                .px(px(5.0))
                .py(px(1.0))
                .rounded_full()
                .bg(match status {
                    SessionStatus::Running => theme.accent_subtle,
                    SessionStatus::Done => theme.success_bg,
                    SessionStatus::Idle => theme.surface,
                })
                .text_size(px(9.0))
                .font_weight(FontWeight::BOLD)
                .text_color(match status {
                    SessionStatus::Running => theme.accent_text,
                    SessionStatus::Done => theme.success,
                    SessionStatus::Idle => theme.text_subtle,
                })
                .child(status_label),
        )
}

fn session_age(updated_ms: i64) -> String {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or_default();
    let elapsed = (now_ms - updated_ms).max(0) / 1000;
    if elapsed < 60 {
        "now".into()
    } else if elapsed < 3600 {
        format!("{}m", elapsed / 60)
    } else if elapsed < 86_400 {
        format!("{}h", elapsed / 3600)
    } else {
        format!("{}d", elapsed / 86_400)
    }
}

fn sidebar_stats(
    session_count: usize,
    active_index: usize,
    theme: &ThemeColors,
) -> impl IntoElement {
    let progress = ((session_count.min(12) as f32) / 12.0 * 100.0).max(8.0);
    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .px(px(12.0))
        .py(px(10.0))
        .border_t_1()
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(
                    Label::new("workspace")
                        .color(theme.text_subtle)
                        .size(LabelSize::Xs),
                )
                .child(
                    Label::new(format!("{} sessions", session_count))
                        .color(theme.text)
                        .size(LabelSize::Xs)
                        .weight(FontWeight::SEMIBOLD),
                ),
        )
        .child(
            div()
                .h(px(3.0))
                .w_full()
                .rounded_full()
                .bg(theme.border)
                .child(
                    div()
                        .h_full()
                        .w(relative(progress / 100.0))
                        .rounded_full()
                        .bg(theme.accent),
                ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(2.0))
                .children((0..14).map(|i| {
                    let intensity = (i + active_index) % 5;
                    div().flex_1().h(px(8.0)).rounded_sm().bg(match intensity {
                        0 => theme.border,
                        1 => theme.info_bg,
                        2 => theme.accent_subtle,
                        3 => theme.accent,
                        _ => theme.accent_hover,
                    })
                })),
        )
}

fn sidebar_footer(theme: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(1.0))
        .px(px(8.0))
        .py(px(6.0))
        .border_t_1()
        .border_color(theme.border)
        .child(footer_item("Agents", "A", theme))
        .child(footer_item("Skills", "S", theme))
        .child(footer_item("Runtime ready", "●", theme))
}

fn footer_item(label: &'static str, icon: &'static str, theme: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .px(px(8.0))
        .py(px(6.0))
        .rounded_sm()
        .text_size(px(11.0))
        .text_color(theme.text_muted)
        .hover(|el| el.bg(theme.overlay).text_color(theme.text))
        .child(
            div()
                .w(px(16.0))
                .text_align(TextAlign::Center)
                .text_color(if label == "Runtime ready" {
                    theme.success
                } else {
                    theme.text_subtle
                })
                .child(icon),
        )
        .child(label)
}
