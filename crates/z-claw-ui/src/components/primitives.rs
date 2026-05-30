use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;
use std::sync::Arc;

// ── Button ──

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    on_click: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync>>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            on_click: None,
        }
    }
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn on_click(
        mut self,
        h: impl Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_click = Some(Arc::new(h));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let bg = match self.variant {
            ButtonVariant::Primary => theme.accent,
            ButtonVariant::Secondary => theme.surface,
            ButtonVariant::Danger => theme.error,
        };
        div()
            .px(px(14.0))
            .py(px(6.0))
            .bg(bg)
            .rounded_md()
            .text_color(theme.background)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .child(self.label)
            .on_mouse_down(MouseButton::Left, {
                let h = self.on_click.clone();
                move |e, w, cx| {
                    if let Some(ref h) = h {
                        h(e, w, cx);
                    }
                }
            })
    }
}

// ── Label ──

#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    color: Option<Rgba>,
    size: LabelSize,
}

#[derive(Clone, Copy)]
pub enum LabelSize {
    Xs,
    Sm,
    Default,
    Lg,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            color: None,
            size: LabelSize::Default,
        }
    }
    pub fn color(mut self, c: Rgba) -> Self {
        self.color = Some(c);
        self
    }
    pub fn size(mut self, s: LabelSize) -> Self {
        self.size = s;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let mut el = div()
            .text_color(self.color.unwrap_or(theme.text))
            .child(self.text);
        match self.size {
            LabelSize::Xs => {
                el = el.text_xs();
            }
            LabelSize::Sm => {
                el = el.text_sm();
            }
            LabelSize::Default => {}
            LabelSize::Lg => {
                el = el.text_lg();
            }
        }
        el
    }
}

// ── TabBar ──

#[derive(IntoElement)]
pub struct TabBar {
    tabs: Vec<TabItem>,
    active: usize,
}

#[derive(Clone)]
pub struct TabItem {
    pub id: String,
    pub label: SharedString,
}

impl TabBar {
    pub fn new(tabs: Vec<TabItem>, active: usize) -> Self {
        Self { tabs, active }
    }
}

impl RenderOnce for TabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        div()
            .flex()
            .flex_row()
            .bg(theme.sidebar_bg)
            .rounded_t_md()
            .children(self.tabs.iter().enumerate().map(|(i, t)| {
                let active = i == self.active;
                div()
                    .px(px(14.0))
                    .py(px(6.0))
                    .text_sm()
                    .bg(if active {
                        theme.background
                    } else {
                        theme.sidebar_bg
                    })
                    .text_color(if active { theme.text } else { theme.text_muted })
                    .rounded_t_md()
                    .cursor_pointer()
                    .child(t.label.clone())
            }))
    }
}
