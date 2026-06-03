use std::sync::Arc;

use crate::components::{Button, ButtonVariant, Label, LabelSize};
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
            .bg(rgba(0x000000bb))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(440.0))
                    .bg(theme.surface)
                    .rounded_md()
                    .border_1()
                    .border_color(theme.warning)
                    .p(px(18.0))
                    .child(
                        div().flex().flex_row().items_center().mb(px(12.0)).child(
                            Label::new("Tool Approval Required")
                                .color(theme.warning)
                                .size(LabelSize::Lg)
                                .weight(FontWeight::SEMIBOLD),
                        ),
                    )
                    .child(
                        div()
                            .mb(px(10.0))
                            .child(Label::new(format!("Tool: {}", req.tool_name))),
                    )
                    .child(
                        div().mb(px(10.0)).child(
                            Label::new(format!("Level: {}", req.security_level))
                                .color(theme.text_muted),
                        ),
                    )
                    .child(
                        div()
                            .mb(px(16.0))
                            .px(px(10.0))
                            .py(px(8.0))
                            .bg(theme.input_bg)
                            .rounded_md()
                            .border_1()
                            .border_color(theme.border)
                            .child(
                                Label::new(format!("Arguments: {}", req.arguments))
                                    .color(theme.text_subtle)
                                    .size(LabelSize::Xs),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .on_mouse_down(MouseButton::Left, {
                                        let h = self.on_deny.clone();
                                        move |e, _, cx| {
                                            if let Some(ref h) = h {
                                                h(e, cx);
                                            }
                                        }
                                    })
                                    .child(Button::new("Deny").variant(ButtonVariant::Secondary)),
                            )
                            .child(
                                div()
                                    .on_mouse_down(MouseButton::Left, {
                                        let h = self.on_approve.clone();
                                        move |e, _, cx| {
                                            if let Some(ref h) = h {
                                                h(e, cx);
                                            }
                                        }
                                    })
                                    .child(Button::new("Approve").variant(ButtonVariant::Primary)),
                            ),
                    ),
            )
    }
}
