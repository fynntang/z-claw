use crate::components::{Label, LabelSize};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;

/// Simple Markdown text renderer. Supports ```code blocks```, **bold**, `inline code`.
pub fn render_markdown(text: &str, cx: &App) -> impl IntoElement {
    let segments = parse_segments(text);
    div()
        .flex()
        .flex_col()
        .children(segments.into_iter().map(|seg| {
            match seg {
                Segment::CodeBlock { language: _, code } => div()
                    .bg(theme_colors(cx).sidebar_bg)
                    .rounded_md()
                    .px(px(12.0))
                    .py(px(8.0))
                    .my_1()
                    .child(Label::new(code).size(LabelSize::Xs)),
                Segment::InlineCode(code) => div()
                    .bg(theme_colors(cx).sidebar_bg)
                    .px(px(4.0))
                    .rounded_sm()
                    .child(
                        Label::new(code)
                            .color(theme_colors(cx).accent)
                            .size(LabelSize::Xs),
                    ),
                Segment::Bold(text) => div().child(Label::new(text).weight(FontWeight::BOLD)),
                Segment::Text(text) => div().child(Label::new(text)),
                Segment::LineBreak => div().h(px(4.0)),
            }
        }))
}

fn theme_colors(cx: &App) -> ThemeColors {
    *cx.global::<ThemeColors>()
}

enum Segment {
    Text(String),
    Bold(String),
    InlineCode(String),
    CodeBlock { language: String, code: String },
    LineBreak,
}

fn parse_segments(text: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let parts: Vec<&str> = text.split("```").collect();
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i % 2 == 1 {
            let (lang, code) = if let Some(nl) = part.find('\n') {
                (part[..nl].trim().to_string(), part[nl + 1..].to_string())
            } else {
                (String::new(), part.to_string())
            };
            segments.push(Segment::CodeBlock {
                language: lang,
                code: code.trim_end().to_string(),
            });
        } else {
            segments.extend(parse_inline(part));
        }
    }
    segments
}

fn parse_inline(text: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut remaining = text;
    while !remaining.is_empty() {
        if let Some(tick) = remaining.find('`') {
            if !remaining[..tick].is_empty() {
                segments.extend(parse_bold(&remaining[..tick]));
            }
            let after = &remaining[tick + 1..];
            if let Some(close) = after.find('`') {
                segments.push(Segment::InlineCode(after[..close].to_string()));
                remaining = &after[close + 1..];
            } else {
                segments.push(Segment::Text(format!("`{after}")));
                remaining = "";
            }
        } else {
            segments.extend(parse_bold(remaining));
            remaining = "";
        }
    }
    segments
}

fn parse_bold(text: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut remaining = text;
    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
            if !remaining[..start].is_empty() {
                segments.push(Segment::Text(remaining[..start].to_string()));
            }
            let after = &remaining[start + 2..];
            if let Some(end) = after.find("**") {
                segments.push(Segment::Bold(after[..end].to_string()));
                remaining = &after[end + 2..];
            } else {
                segments.push(Segment::Text(format!("**{after}")));
                remaining = "";
            }
        } else {
            segments.push(Segment::Text(remaining.to_string()));
            remaining = "";
        }
    }
    segments
}
