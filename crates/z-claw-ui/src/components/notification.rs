use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct ToastNotification {
    pub message: SharedString,
    pub kind: ToastKind,
}

#[derive(Clone)]
pub enum ToastKind {
    Info,
    Success,
    Error,
    Warning,
}

impl RenderOnce for ToastNotification {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let (bg, border) = match self.kind {
            ToastKind::Info => (theme.surface, theme.accent),
            ToastKind::Success => (theme.surface, theme.success),
            ToastKind::Error => (theme.surface, theme.error),
            ToastKind::Warning => (theme.surface, theme.warning),
        };
        div()
            .absolute()
            .top(px(12.0))
            .right(px(12.0))
            .px(px(16.0))
            .py(px(10.0))
            .bg(bg)
            .rounded_md()
            .border_1()
            .border_color(border)
            .shadow_lg()
            .text_sm()
            .text_color(theme.text)
            .child(self.message)
    }
}
