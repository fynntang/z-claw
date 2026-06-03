use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct TitleBar {
    title: SharedString,
    subtitle: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl TitleBar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            children: Vec::new(),
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let is_macos = cfg!(target_os = "macos");

        div()
            .id("z-claw-custom-title-bar")
            .h(px(38.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .bg(theme.sidebar_bg)
            .border_b_1()
            .border_color(theme.border)
            .window_control_area(WindowControlArea::Drag)
            .on_click(|event, window, _| {
                if event.click_count() == 2 {
                    if cfg!(target_os = "macos") {
                        window.titlebar_double_click();
                    } else {
                        window.zoom_window();
                    }
                }
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .pl(if is_macos { px(78.0) } else { px(12.0) })
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded_md()
                            .bg(theme.accent)
                            .border_1()
                            .border_color(theme.accent),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text)
                                    .child(self.title),
                            )
                            .when_some(self.subtitle, |el, subtitle| {
                                el.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(subtitle),
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .children(self.children),
            )
            .child(window_controls(window, theme))
    }
}

fn window_controls(window: &mut Window, theme: &ThemeColors) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .justify_center()
        .content_stretch()
        .max_h(px(38.0))
        .min_h(px(38.0))
        .when(cfg!(target_os = "windows"), |el| {
            el.font_family(windows_caption_font())
                .child(WindowsCaptionButton::Minimize)
                .child(if window.is_maximized() {
                    WindowsCaptionButton::Restore
                } else {
                    WindowsCaptionButton::Maximize
                })
                .child(WindowsCaptionButton::Close)
        })
        .when(
            !cfg!(target_os = "windows") && !cfg!(target_os = "macos"),
            |el| {
                el.child(generic_caption_button(
                    "minimize-window",
                    "-",
                    WindowControlArea::Min,
                    theme,
                    |window, _| window.minimize_window(),
                ))
                .child(generic_caption_button(
                    "maximize-window",
                    if window.is_maximized() { "[]" } else { "[ ]" },
                    WindowControlArea::Max,
                    theme,
                    |window, _| window.zoom_window(),
                ))
                .child(generic_caption_button(
                    "close-window",
                    "x",
                    WindowControlArea::Close,
                    theme,
                    |window, _| window.remove_window(),
                ))
            },
        )
}

fn windows_caption_font() -> &'static str {
    "Segoe Fluent Icons"
}

#[derive(IntoElement)]
enum WindowsCaptionButton {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowsCaptionButton {
    fn id(&self) -> &'static str {
        match self {
            Self::Minimize => "windows-minimize-window",
            Self::Restore => "windows-restore-window",
            Self::Maximize => "windows-maximize-window",
            Self::Close => "windows-close-window",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Minimize => "\u{e921}",
            Self::Restore => "\u{e923}",
            Self::Maximize => "\u{e922}",
            Self::Close => "\u{e8bb}",
        }
    }

    fn control_area(&self) -> WindowControlArea {
        match self {
            Self::Minimize => WindowControlArea::Min,
            Self::Restore | Self::Maximize => WindowControlArea::Max,
            Self::Close => WindowControlArea::Close,
        }
    }
}

impl RenderOnce for WindowsCaptionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        let is_close = matches!(self, Self::Close);

        div()
            .id(self.id())
            .w(px(36.0))
            .h_full()
            .flex()
            .occlude()
            .items_center()
            .justify_center()
            .content_center()
            .text_size(px(10.0))
            .text_color(theme.text_muted)
            .window_control_area(self.control_area())
            .hover(move |el| {
                if is_close {
                    el.bg(theme.error).text_color(white())
                } else {
                    el.bg(theme.overlay).text_color(theme.text)
                }
            })
            .active(move |el| {
                if is_close {
                    el.bg(theme.error).text_color(white())
                } else {
                    el.bg(theme.input_bg).text_color(theme.text)
                }
            })
            .child(self.icon())
    }
}

fn generic_caption_button(
    id: &'static str,
    label: impl Into<SharedString>,
    control_area: WindowControlArea,
    theme: &ThemeColors,
    action: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let is_close = control_area == WindowControlArea::Close;
    let theme = *theme;

    div()
        .id(id)
        .w(px(44.0))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(theme.text_muted)
        .window_control_area(control_area)
        .hover(move |el| {
            if is_close {
                el.bg(theme.error).text_color(theme.background)
            } else {
                el.bg(theme.overlay).text_color(theme.text)
            }
        })
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            action(window, cx);
        })
        .child(label.into())
}
