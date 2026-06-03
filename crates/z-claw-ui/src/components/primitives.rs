use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;
use std::rc::Rc;

// ── Button ──

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    disabled: bool,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            on_click: None,
            disabled: false,
        }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        let (bg, text, border) = match self.variant {
            ButtonVariant::Primary => (theme.accent, theme.background, theme.accent),
            ButtonVariant::Secondary => (theme.overlay, theme.text, theme.border),
            ButtonVariant::Danger => (theme.error, theme.background, theme.error),
        };

        let (bg, text, border) = if self.disabled {
            (theme.surface, theme.text_subtle, theme.border)
        } else {
            (bg, text, border)
        };

        let interactive = !self.disabled && self.on_click.is_some();

        div()
            .id(ElementId::Name(self.label.clone().into()))
            .min_h(px(36.0))
            .px(px(14.0))
            .py(px(8.0))
            .bg(bg)
            .rounded_md()
            .border_1()
            .border_color(border)
            .text_color(text)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .when(interactive, |el| el.cursor_pointer())
            .hover(|el| {
                if self.disabled {
                    el
                } else {
                    el.bg(match self.variant {
                        ButtonVariant::Primary => theme.accent,
                        ButtonVariant::Secondary => theme.overlay,
                        ButtonVariant::Danger => theme.error,
                    })
                }
            })
            .child(self.label.clone())
            .when_some(self.on_click.filter(|_| !self.disabled), |el, h| {
                el.on_click(move |e, w, cx| h(e, w, cx))
            })
    }
}

// ── Label ──

#[derive(Clone, Copy)]
pub enum LabelSize {
    Xs,
    Sm,
    Default,
    Lg,
}

#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    color: Option<Rgba>,
    size: LabelSize,
    weight: Option<FontWeight>,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            color: None,
            size: LabelSize::Default,
            weight: None,
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

    pub fn weight(mut self, w: FontWeight) -> Self {
        self.weight = Some(w);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        let mut el = div()
            .text_color(self.color.unwrap_or(theme.text))
            .line_height(px(20.0))
            .child(self.text.clone());

        if let Some(w) = self.weight {
            el = el.font_weight(w);
        }

        match self.size {
            LabelSize::Xs => el.text_xs(),
            LabelSize::Sm => el.text_sm(),
            LabelSize::Default => el,
            LabelSize::Lg => el.text_lg(),
        }
    }
}

// ── TabBar ──

#[derive(Clone)]
pub struct TabItem {
    pub id: String,
    pub label: SharedString,
}

#[derive(IntoElement)]
pub struct TabBar {
    tabs: Vec<TabItem>,
    active: usize,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
}

impl TabBar {
    pub fn new(tabs: Vec<TabItem>, active: usize) -> Self {
        Self {
            tabs,
            active,
            on_click: None,
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for TabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let on_click = self.on_click;
        let active = self.active;
        div()
            .flex()
            .flex_row()
            .bg(theme.sidebar_bg)
            .rounded_t_md()
            .children(self.tabs.iter().enumerate().map(move |(i, t)| {
                let is_active = i == active;
                let h = on_click.clone();
                div()
                    .id(ElementId::Name(t.id.clone().into()))
                    .px(px(14.0))
                    .py(px(6.0))
                    .text_sm()
                    .bg(if is_active {
                        theme.background
                    } else {
                        theme.sidebar_bg
                    })
                    .text_color(if is_active {
                        theme.text
                    } else {
                        theme.text_muted
                    })
                    .rounded_t_md()
                    .cursor_pointer()
                    .child(t.label.clone())
                    .when_some(h, |el, h| el.on_click(move |_, w, cx| h(&i, w, cx)))
            }))
    }
}
