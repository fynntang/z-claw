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

/// Which settings field is currently being edited.
#[derive(Clone, PartialEq)]
pub enum SettingsField {
    Model,
    Endpoint,
    ApiKey,
}

#[derive(IntoElement)]
pub struct SettingsPanel {
    pub settings: ProviderSettings,
    pub editing_field: Option<SettingsField>,
    pub on_close: Option<Arc<dyn Fn(&MouseDownEvent, &mut App) + Send + Sync>>,
    pub on_save: Option<Arc<dyn Fn(ProviderSettings, &mut App) + Send + Sync>>,
    pub on_provider_change: Option<Arc<dyn Fn(String, &mut App) + Send + Sync>>,
    pub on_field_click: Option<Arc<dyn Fn(SettingsField, &mut App) + Send + Sync>>,
}

impl SettingsPanel {
    pub fn new(settings: ProviderSettings, editing_field: Option<SettingsField>) -> Self {
        Self {
            settings,
            editing_field,
            on_close: None,
            on_save: None,
            on_provider_change: None,
            on_field_click: None,
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

    pub fn on_field_click(
        mut self,
        h: impl Fn(SettingsField, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_field_click = Some(Arc::new(h));
        self
    }
}

impl RenderOnce for SettingsPanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeColors>();

        div()
            .absolute()
            .size_full()
            .bg(rgba(0x000000bb))
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
                    .w(px(460.0))
                    .bg(theme.surface)
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .p(px(18.0))
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .mb(px(18.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        Label::new("Settings")
                                            .color(theme.text)
                                            .size(LabelSize::Lg)
                                            .weight(FontWeight::SEMIBOLD),
                                    )
                                    .child(
                                        Label::new("Provider and model")
                                            .color(theme.text_subtle)
                                            .size(LabelSize::Xs),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded_md()
                                    .bg(theme.overlay)
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .child("Close")
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
                        div().mb(px(8.0)).child(
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
                                        .border_1()
                                        .border_color(if active {
                                            theme.accent
                                        } else {
                                            theme.border
                                        })
                                        .bg(if active {
                                            theme.overlay
                                        } else {
                                            theme.input_bg
                                        })
                                        .text_color(if active {
                                            theme.text
                                        } else {
                                            theme.text_muted
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
                            .mt(px(16.0))
                            .child(editable_row(
                                "Model",
                                &self.settings.model,
                                SettingsField::Model,
                                false,
                                &self.editing_field,
                                &self.on_field_click,
                                theme,
                            ))
                            .child(editable_row(
                                "Endpoint",
                                &self.settings.endpoint,
                                SettingsField::Endpoint,
                                false,
                                &self.editing_field,
                                &self.on_field_click,
                                theme,
                            ))
                            .child(editable_row(
                                "API Key",
                                &self.settings.api_key,
                                SettingsField::ApiKey,
                                true,
                                &self.editing_field,
                                &self.on_field_click,
                                theme,
                            )),
                    )
                    .child(
                        div().mt(px(18.0)).flex().flex_row().justify_end().child(
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

fn editable_row(
    label: &str,
    value: &str,
    field: SettingsField,
    is_secret: bool,
    active_field: &Option<SettingsField>,
    on_click: &Option<Arc<dyn Fn(SettingsField, &mut App) + Send + Sync>>,
    theme: &ThemeColors,
) -> impl IntoElement {
    let label = label.to_string();
    let theme = *theme;
    let is_active = active_field.as_ref() == Some(&field);
    let border_color = if is_active {
        theme.accent
    } else {
        theme.border
    };

    let display_value = if is_active {
        format!("{value} |")
    } else if is_secret {
        mask_key(value)
    } else {
        value.to_string()
    };

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
                .px(px(10.0))
                .py(px(8.0))
                .bg(theme.input_bg)
                .rounded_md()
                .border_1()
                .border_color(border_color)
                .cursor_pointer()
                .child(
                    Label::new(display_value)
                        .color(theme.text)
                        .size(LabelSize::Sm),
                )
                .on_mouse_down(MouseButton::Left, {
                    let h = on_click.clone();
                    let field = field.clone();
                    move |_, _, cx| {
                        if let Some(ref h) = h {
                            h(field.clone(), cx);
                        }
                    }
                }),
        )
}

fn mask_key(key: &str) -> String {
    if key.len() <= 4 {
        "****".into()
    } else {
        format!("{}****{}", &key[..4], &key[key.len() - 4..])
    }
}
