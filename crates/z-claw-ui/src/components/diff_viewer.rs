use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(IntoElement)]
pub struct DiffViewer {
    pub file_path: SharedString,
    pub diff_lines: Vec<String>,
}

impl DiffViewer {
    pub fn new(file_path: impl Into<SharedString>, diff_text: &str) -> Self {
        Self {
            file_path: file_path.into(),
            diff_lines: diff_text.lines().map(|l| l.to_string()).collect(),
        }
    }
}

impl RenderOnce for DiffViewer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();
        div()
            .flex()
            .flex_col()
            .bg(theme.sidebar_bg)
            .rounded_md()
            .border_1()
            .border_color(theme.border)
            .mb_2()
            .child(
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .bg(theme.surface)
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(format!("File: {}", self.file_path)),
            )
            .child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .text_xs()
                    .children(self.diff_lines.iter().map(|line| {
                        let (color, prefix) = if line.starts_with('+') {
                            (theme.success, "+")
                        } else if line.starts_with('-') {
                            (theme.error, "-")
                        } else {
                            (theme.text_muted, " ")
                        };
                        div()
                            .py(px(1.0))
                            .text_color(color)
                            .child(format!("{prefix} {line}"))
                    })),
            )
    }
}
