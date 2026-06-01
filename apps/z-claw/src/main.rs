use std::ops::Range;

use gpui::prelude::*;
use gpui::*;
use gpui_platform::application;
use z_claw_assets::Assets;
use z_claw_ui::components::{Button, ButtonVariant};
use z_claw_ui::views::settings::{ProviderSettings, SettingsField};
use z_claw_ui::views::sidebar::SessionSummary;
use z_claw_ui::{AppModel, ApprovalDialog, ChatView, SettingsPanel, Sidebar, ThemeColors};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("z_claw=info")
        .init();

    tracing::info!("Starting z-claw desktop app");

    let assets = Assets;

    application().run(move |cx: &mut App| {
        if let Err(e) = assets.load_fonts(cx) {
            tracing::warn!("Failed to load fonts: {e}");
        }

        cx.set_global(ThemeColors::catppuccin_mocha());

        let app_model = cx.new(|_cx| AppModel::new());

        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| MainWindow::new(app_model.clone(), cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}

// ── MainWindow ──

struct MainWindow {
    app_model: Entity<AppModel>,
    input_text: SharedString,
    focus_handle: FocusHandle,
    sessions: Vec<SessionSummary>,
    show_settings: bool,
    current_settings: ProviderSettings,
    editing_field: Option<SettingsField>,
}

impl MainWindow {
    fn new(app_model: Entity<AppModel>, cx: &mut Context<Self>) -> Self {
        let _ = cx.observe(&app_model, |_, _, cx| cx.notify());

        // Load sessions from SQLite asynchronously
        let memory = app_model.read(cx).memory.clone();
        cx.spawn(
            async move |this: WeakEntity<MainWindow>, cx: &mut AsyncApp| {
                if let Ok(list) = memory.list_sessions().await {
                    let sessions = list
                        .into_iter()
                        .map(|(id, title, updated_ms)| SessionSummary {
                            id,
                            title,
                            updated_ms,
                        })
                        .collect::<Vec<_>>();
                    _ = this.update(cx, |this, cx| {
                        this.sessions = sessions;
                        cx.notify();
                    });
                }
            },
        )
        .detach();

        Self {
            app_model,
            input_text: SharedString::default(),
            focus_handle: cx.focus_handle(),
            sessions: Vec::new(),
            show_settings: false,
            current_settings: ProviderSettings {
                provider_type: "ollama".into(),
                model: "llama3".into(),
                endpoint: "http://localhost:11434/v1".into(),
                api_key: "".into(),
            },
            editing_field: None,
        }
    }

    fn submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let text = std::mem::take(&mut self.input_text);
        if text.trim().is_empty() {
            return;
        }
        self.app_model.update(cx, |model, cx| {
            model.send_text(&text, cx);
        });
        cx.notify();
    }

    /// Apply a character to the currently editing settings field.
    fn apply_to_field(&mut self, text: &str, cx: &mut Context<Self>) {
        if let Some(ref field) = self.editing_field {
            match field {
                SettingsField::Model => {
                    self.current_settings.model.push_str(text);
                }
                SettingsField::Endpoint => {
                    self.current_settings.endpoint.push_str(text);
                }
                SettingsField::ApiKey => {
                    self.current_settings.api_key.push_str(text);
                }
            }
            cx.notify();
        }
    }

    /// Backspace in the currently editing settings field.
    fn backspace_field(&mut self, cx: &mut Context<Self>) {
        if let Some(ref field) = self.editing_field {
            match field {
                SettingsField::Model => {
                    self.current_settings.model.pop();
                }
                SettingsField::Endpoint => {
                    self.current_settings.endpoint.pop();
                }
                SettingsField::ApiKey => {
                    self.current_settings.api_key.pop();
                }
            }
            cx.notify();
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app_model.update(cx, |model, cx| model.poll_events(cx));

        self.focus_handle.clone().focus(window, cx);

        let model = self.app_model.read(cx);
        let theme = *cx.global::<ThemeColors>();
        let messages = model.messages.clone();
        let streaming = model.streaming;
        let pending_approval = model.pending_approval.clone();

        let sessions = self.sessions.clone();

        // Sync typed text for send handler
        self.app_model.update(cx, |model, _cx| {
            model.input_text = self.input_text.to_string();
        });

        let app_model_new = self.app_model.clone();
        let show_settings = self.show_settings;
        let has_text = !self.input_text.trim().is_empty();
        let can_send = has_text && !streaming;
        let cursor = "|";

        let mut root = div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .child(
                Sidebar::new()
                    .with_sessions(sessions)
                    .on_new_session(move |_, _, cx| {
                        app_model_new.update(cx, |model, cx| {
                            model.new_session();
                            cx.notify();
                        });
                    }),
            )
            .child(
                // Main content: chat + input
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .bg(theme.background)
                    .child(
                        // Message area
                        ChatView { messages },
                    )
                    .child(
                        // Visible text input bar
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .px(px(16.0))
                            .py(px(10.0))
                            .bg(theme.surface)
                            .border_t_1()
                            .border_color(theme.border)
                            .child(
                                div()
                                    .flex_1()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .bg(theme.background)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(if self.editing_field.is_none() {
                                        theme.border
                                    } else {
                                        theme.text_subtle
                                    })
                                    .text_sm()
                                    .text_color(if self.input_text.is_empty() {
                                        theme.text_subtle
                                    } else {
                                        theme.text
                                    })
                                    .child(if self.input_text.is_empty() {
                                        SharedString::from("Type a message...")
                                    } else {
                                        format!("{}{}", &self.input_text, cursor).into()
                                    }),
                            )
                            .child(
                                div()
                                    .ml(px(8.0))
                                    .on_mouse_down(MouseButton::Left, {
                                        let app_model = self.app_model.clone();
                                        move |_, _, cx| {
                                            app_model.update(cx, |model, cx| {
                                                model.send_message(cx);
                                            });
                                        }
                                    })
                                    .child(
                                        Button::new("Send")
                                            .variant(if can_send {
                                                ButtonVariant::Primary
                                            } else {
                                                ButtonVariant::Secondary
                                            })
                                            .disabled(!can_send),
                                    ),
                            ),
                    ),
            );

        if let Some(req) = pending_approval {
            let e1 = cx.entity();
            let e2 = e1.clone();
            root = root.child(
                ApprovalDialog::new(req)
                    .on_approve(move |_, cx| {
                        e1.update(cx, |this, cx| {
                            this.app_model.update(cx, |m, _| m.clear_approval());
                            cx.notify();
                        });
                    })
                    .on_deny(move |_, cx| {
                        e2.update(cx, |this, cx| {
                            this.app_model.update(cx, |m, _| m.clear_approval());
                            cx.notify();
                        });
                    }),
            );
        }

        if show_settings {
            let current = self.current_settings.clone();
            let editing = self.editing_field.clone();
            root = root.child(
                SettingsPanel::new(current, editing)
                    .on_provider_change({
                        let entity = cx.entity();
                        move |provider, cx| {
                            entity.update(cx, |this: &mut MainWindow, cx| {
                                this.current_settings.provider_type = provider;
                                cx.notify();
                            });
                        }
                    })
                    .on_field_click({
                        let entity = cx.entity();
                        move |field, cx| {
                            entity.update(cx, |this: &mut MainWindow, cx| {
                                this.editing_field = Some(field);
                                cx.notify();
                            });
                        }
                    })
                    .on_close({
                        let entity = cx.entity();
                        move |_, cx| {
                            entity.update(cx, |this: &mut MainWindow, cx| {
                                this.show_settings = false;
                                this.editing_field = None;
                                cx.notify();
                            });
                        }
                    }),
            );
        }

        root
    }
}

// ── Text input ──

impl EntityInputHandler for MainWindow {
    fn text_for_range(
        &mut self,
        _range: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        if self.editing_field.is_some() {
            let value = match self.editing_field.as_ref().unwrap() {
                SettingsField::Model => &self.current_settings.model,
                SettingsField::Endpoint => &self.current_settings.endpoint,
                SettingsField::ApiKey => &self.current_settings.api_key,
            };
            Some(value.clone())
        } else {
            Some(self.input_text.to_string())
        }
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let len = if let Some(ref field) = self.editing_field {
            match field {
                SettingsField::Model => self.current_settings.model.len(),
                SettingsField::Endpoint => self.current_settings.endpoint.len(),
                SettingsField::ApiKey => self.current_settings.api_key.len(),
            }
        } else {
            self.input_text.len()
        };
        Some(UTF16Selection {
            range: len..len,
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        _range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editing_field.is_some() {
            // Editing a settings field
            if text == "\r" || text == "\n" || text == "\r\n" {
                // Enter confirms the field edit, clear focus back to chat
                self.editing_field = None;
                cx.notify();
                return;
            }
            if text.is_empty() {
                // Backspace
                self.backspace_field(cx);
            } else {
                self.apply_to_field(text, cx);
            }
            return;
        }

        // Editing chat input
        if text == "\n" || text == "\r" || text == "\r\n" {
            self.submit(window, cx);
            return;
        }
        if text.is_empty() {
            let mut s = self.input_text.to_string();
            s.pop();
            self.input_text = s.into();
        } else {
            self.input_text = (self.input_text.to_string() + text).into();
        }
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editing_field.is_some() {
            if new_text.is_empty() {
                self.backspace_field(cx);
            } else {
                self.apply_to_field(new_text, cx);
            }
            return;
        }
        if new_text.is_empty() {
            let mut s = self.input_text.to_string();
            s.pop();
            self.input_text = s.into();
        } else {
            self.input_text = (self.input_text.to_string() + new_text).into();
        }
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let len = if let Some(ref field) = self.editing_field {
            match field {
                SettingsField::Model => self.current_settings.model.len(),
                SettingsField::Endpoint => self.current_settings.endpoint.len(),
                SettingsField::ApiKey => self.current_settings.api_key.len(),
            }
        } else {
            self.input_text.len()
        };
        Some(len)
    }

    fn accepts_text_input(&self, _window: &mut Window, _cx: &mut Context<Self>) -> bool {
        true
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
