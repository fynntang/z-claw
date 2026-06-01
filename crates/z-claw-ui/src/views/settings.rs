use std::sync::Arc;

use crate::components::{Button, ButtonVariant, Label, LabelSize};
use crate::theme::ThemeColors;
use gpui::prelude::*;
use gpui::*;
use gpui_macros::IntoElement;

#[derive(Debug, Clone)]
pub struct ProviderSettings {
    pub provider_type: String,
    pub model: String,
    pub endpoint: String,
    pub api_key: String,
}

#[derive(IntoElement)]
pub struct SettingsPanel {
    pub settings: ProviderSettings,
    pub on_close: Option<Arc<dyn Fn(&MouseDownEvent, &mut App) + Send + Sync>>,
    pub on_save: Option<Arc<dyn Fn(ProviderSettings, &mut App) + Send + Sync>>,
    pub on_provider_change: Option<Arc<dyn Fn(String, &mut App) + Send + Sync>>,
}

impl SettingsPanel {
    pub fn new(settings: ProviderSettings) -> Self {
        Self {
            settings,
            on_close: None,
            on_save: None,
            on_provider_change: None,
        }
    }

    pub fn on_close(
        mut self,
        h: impl Fn(&MouseDownEvent, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_close = Some(Arc::new(h));
        self
    }

    pub fn on_save(
        mut self,
        h: impl Fn(ProviderSettings, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_save = Some(Arc::new(h));
        self
    }

    pub fn on_provider_change(
        mut self,
        h: impl Fn(String, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_provider_change = Some(Arc::new(h));
        self
    }
}

impl RenderOnce for SettingsPanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        div()
            .absolute()
            .size_full()
            .bg(rgba(0x000000aa))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, {
                let h = self.on_close.clone();
                move |event, _, cx| {
                    if let Some(ref h) = h {
                        h(event, cx);
                    }
                }
            })
            .child(
                div()
                    .w(px(420.0))
                    .bg(theme.background)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border)
                    .p(px(20.0))
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .mb(px(16.0))
                            .child(Label::new("Settings").color(theme.text).size(LabelSize::Lg))
                            .child(
                                div()
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .child("X")
                                    .on_mouse_down(MouseButton::Left, {
                                        let h = self.on_close.clone();
                                        move |event, _, cx| {
                                            if let Some(ref h) = h {
                                                h(event, cx);
                                            }
                                        }
                                    }),
                            ),
                    )
                    .child(
                        div().mb(px(16.0)).child(
                            div().mb(px(6.0)).child(
                                Label::new("Provider")
                                    .color(theme.text_subtle)
                                    .size(LabelSize::Xs),
                            ),
                        ),
                    )
                    .child(
                        div().flex().flex_col().gap_2().children(
                            ["ollama", "openai", "deepseek", "anthropic"]
                                .iter()
                                .map(|&p| {
                                    let active = self.settings.provider_type == p;
                                    let provider = p.to_string();
                                    div()
                                        .px(px(12.0))
                                        .py(px(8.0))
                                        .rounded_md()
                                        .bg(if active { theme.accent } else { theme.surface })
                                        .text_color(if active {
                                            theme.background
                                        } else {
                                            theme.text
                                        })
                                        .cursor_pointer()
                                        .child(p)
                                        .on_mouse_down(MouseButton::Left, {
                                            let h = self.on_provider_change.clone();
                                            let provider = provider.clone();
                                            move |_, _, cx| {
                                                if let Some(ref h) = h {
                                                    h(provider.clone(), cx);
                                                }
                                            }
                                        })
                                }),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(info_row("Model", &self.settings.model, theme))
                            .child(info_row("Endpoint", &self.settings.endpoint, theme))
                            .child(info_row(
                                "API Key",
                                &mask_key(&self.settings.api_key),
                                theme,
                            )),
                    )
                    .child(
                        div().mt(px(16.0)).flex().flex_row().justify_end().child(
                            div()
                                .on_mouse_down(MouseButton::Left, {
                                    let s = self.settings.clone();
                                    let h = self.on_save.clone();
                                    move |_, _, cx| {
                                        if let Some(ref h) = h {
                                            h(s.clone(), cx);
                                        }
                                    }
                                })
                                .child(Button::new("Save & Apply").variant(ButtonVariant::Primary)),
                        ),
                    ),
            )
    }
}

fn info_row(label: &str, value: &str, theme: &ThemeColors) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    let theme = *theme;
    div()
        .flex()
        .flex_col()
        .child(
            div().child(
                Label::new(label)
                    .color(theme.text_subtle)
                    .size(LabelSize::Xs),
            ),
        )
        .child(
            div()
                .mt(px(2.0))
                .px(px(8.0))
                .py(px(4.0))
                .bg(theme.background)
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .child(Label::new(format!("{value} |")).color(theme.text)),
        )
}

fn mask_key(key: &str) -> String {
    if key.len() <= 4 {
        "****".into()
    } else {
        format!("{}****{}", &key[..4], &key[key.len() - 4..])
    }
}
