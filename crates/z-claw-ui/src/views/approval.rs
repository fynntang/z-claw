use std::sync::Arc;

use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub call_id: String,
    pub tool_name: String,
    pub arguments: String,
    pub security_level: String,
}

#[derive(IntoElement)]
pub struct ApprovalDialog {
    pub request: ApprovalRequest,
    pub on_approve: Option<Arc<dyn Fn(&MouseDownEvent, &mut App) + Send + Sync>>,
    pub on_deny: Option<Arc<dyn Fn(&MouseDownEvent, &mut App) + Send + Sync>>,
}

impl ApprovalDialog {
    pub fn new(request: ApprovalRequest) -> Self {
        Self {
            request,
            on_approve: None,
            on_deny: None,
        }
    }

    pub fn on_approve(
        mut self,
        h: impl Fn(&MouseDownEvent, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_approve = Some(Arc::new(h));
        self
    }

    pub fn on_deny(
        mut self,
        h: impl Fn(&MouseDownEvent, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_deny = Some(Arc::new(h));
        self
    }
}

impl RenderOnce for ApprovalDialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let req = &self.request;

        div()
            .absolute()
            .size_full()
            .bg(rgba(0x000000aa))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(400.0))
                    .bg(theme.background)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.warning)
                    .p(px(20.0))
                    .child(
                        div().flex().flex_row().items_center().mb(px(12.0)).child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.warning)
                                .child("Tool Approval Required"),
                        ),
                    )
                    .child(
                        div()
                            .mb(px(10.0))
                            .text_sm()
                            .text_color(theme.text)
                            .child(format!("Tool: {}", req.tool_name)),
                    )
                    .child(
                        div()
                            .mb(px(10.0))
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child(format!("Level: {}", req.security_level)),
                    )
                    .child(
                        div()
                            .mb(px(16.0))
                            .px(px(10.0))
                            .py(px(8.0))
                            .bg(theme.sidebar_bg)
                            .rounded_md()
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .child(format!("Arguments: {}", req.arguments)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .px(px(14.0))
                                    .py(px(6.0))
                                    .bg(theme.surface)
                                    .rounded_md()
                                    .text_color(theme.text)
                                    .text_sm()
                                    .cursor_pointer()
                                    .child("Deny")
                                    .on_mouse_down(MouseButton::Left, {
                                        let h = self.on_deny.clone();
                                        move |e, _, cx| {
                                            if let Some(ref h) = h {
                                                h(e, cx);
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .px(px(14.0))
                                    .py(px(6.0))
                                    .bg(theme.accent)
                                    .rounded_md()
                                    .text_color(theme.background)
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .child("Approve")
                                    .on_mouse_down(MouseButton::Left, {
                                        let h = self.on_approve.clone();
                                        move |e, _, cx| {
                                            if let Some(ref h) = h {
                                                h(e, cx);
                                            }
                                        }
                                    }),
                            ),
                    ),
            )
    }
}
