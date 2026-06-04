use std::ops::Range;

use gpui::prelude::*;
use gpui::*;
use gpui_platform::application;
use z_claw_assets::Assets;
use z_claw_ui::components::{Button, ButtonVariant, TitleBar};
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

        let bounds = Bounds::centered(None, size(px(1366.0), px(769.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("z-claw".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
                }),
                window_decorations: Some(WindowDecorations::Client),
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
    marked_range: Option<Range<usize>>,
    focus_handle: FocusHandle,
    sessions: Vec<SessionSummary>,
    focused_once: bool,
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
            marked_range: None,
            focus_handle: cx.focus_handle(),
            sessions: Vec::new(),
            focused_once: false,
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

    fn upsert_current_session(&mut self, cx: &mut Context<Self>) {
        let session_id = self.app_model.read(cx).session_id.clone();
        if self.sessions.iter().any(|session| session.id == session_id) {
            return;
        }

        let updated_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or_default();

        self.sessions.insert(
            0,
            SessionSummary {
                id: session_id,
                title: "New Session".into(),
                updated_ms,
            },
        );
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.input_text.trim().is_empty() || self.app_model.read(cx).streaming {
            return;
        }
        let text = std::mem::take(&mut self.input_text);
        self.marked_range = None;
        self.app_model.update(cx, |model, cx| {
            model.send_text(&text, cx);
        });
        self.focus_handle.clone().focus(window, cx);
        self.focused_once = true;
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

        if !self.focused_once && !self.show_settings {
            self.focus_handle.clone().focus(window, cx);
            self.focused_once = true;
        }

        let (messages, streaming, pending_approval, current_session_id) = {
            let model = self.app_model.read(cx);
            (
                model.messages.clone(),
                model.streaming,
                model.pending_approval.clone(),
                model.session_id.clone(),
            )
        };
        let theme = *cx.global::<ThemeColors>();

        let sessions = self.sessions.clone();

        // Sync typed text for send handler
        self.app_model.update(cx, |model, _cx| {
            model.input_text = self.input_text.to_string();
        });

        let show_settings = self.show_settings;
        let has_text = !self.input_text.trim().is_empty();
        let can_send = has_text && !streaming;
        let current_session_short = current_session_id.chars().take(8).collect::<String>();

        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .key_context("MainWindow")
            .bg(theme.background)
            .on_key_down({
                let entity = cx.entity();
                move |event: &KeyDownEvent, _window, _cx: &mut App| {
                    if event.keystroke.key == "backspace" || event.keystroke.key == "delete" {
                        entity.update(_cx, |this: &mut MainWindow, cx| {
                            if this.editing_field.is_some() {
                                this.backspace_field(cx);
                            } else if !this.input_text.is_empty() {
                                let mut s = this.input_text.to_string();
                                s.pop();
                                this.input_text = s.into();
                                cx.notify();
                            }
                        });
                    }
                }
            })
            .child(TitleBar::new("z-claw").subtitle("local agent workspace"))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .bg(theme.background)
                    .child(
                        Sidebar::new()
                            .with_sessions(sessions)
                            .with_active_session(Some(current_session_id))
                            .on_new_session({
                                let app_model = self.app_model.clone();
                                let entity = cx.entity();
                                move |_, window, cx| {
                                    app_model.update(cx, |model, cx| {
                                        model.new_session();
                                        cx.notify();
                                    });
                                    entity.update(cx, |this: &mut MainWindow, cx| {
                                        this.input_text = SharedString::default();
                                        this.marked_range = None;
                                        this.upsert_current_session(cx);
                                        this.focus_handle.clone().focus(window, cx);
                                        cx.notify();
                                    });
                                }
                            })
                            .on_select_session({
                                let app_model = self.app_model.clone();
                                move |session_id, _, cx| {
                                    app_model.update(cx, |model, cx| {
                                        model.switch_session(session_id);
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(theme.background)
                            .child(
                                div()
                                    .h(px(40.0))
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .px(px(14.0))
                                    .gap(px(8.0))
                                    .border_b_1()
                                    .border_color(theme.border)
                                    .bg(theme.background)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .max_w(px(220.0))
                                                    .text_size(px(12.0))
                                                    .text_color(theme.text)
                                                    .font_weight(FontWeight::BOLD)
                                                    .child("Workspace"),
                                            )
                                            .child(
                                                div()
                                                    .px(px(6.0))
                                                    .py(px(2.0))
                                                    .rounded_sm()
                                                    .border_1()
                                                    .border_color(theme.border)
                                                    .bg(theme.sidebar_bg)
                                                    .text_size(px(10.0))
                                                    .text_color(theme.text_subtle)
                                                    .child(current_session_short.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(if streaming {
                                                        theme.info
                                                    } else {
                                                        theme.text_subtle
                                                    })
                                                    .child(if streaming {
                                                        "Responding"
                                                    } else {
                                                        "Idle"
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(toolbar_pill("llama3", &theme))
                                            .child(toolbar_pill("18% ctx", &theme))
                                            .child(
                                                Button::new("Settings")
                                                    .variant(ButtonVariant::Secondary)
                                                    .on_click({
                                                        let entity = cx.entity();
                                                        move |_, _, cx| {
                                                            entity.update(cx, |this, cx| {
                                                                this.show_settings = true;
                                                                this.editing_field = None;
                                                                cx.notify();
                                                            });
                                                            cx.stop_propagation();
                                                        }
                                                    }),
                                            ),
                                    ),
                            )
                            .child(ChatView { messages })
                            .child(
                                div()
                                    .px(px(20.0))
                                    .py(px(12.0))
                                    .bg(theme.background)
                                    .border_t_1()
                                    .border_color(theme.border)
                                    .child(
                                        div()
                                            .mx_auto()
                                            .max_w(px(860.0))
                                            .w_full()
                                            .flex()
                                            .flex_col()
                                            .gap(px(6.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_end()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .min_h(px(38.0))
                                                            .px(px(10.0))
                                                            .py(px(6.0))
                                                            .bg(theme.sidebar_bg)
                                                            .rounded_md()
                                                            .border_1()
                                                            .border_color(
                                                                if self.editing_field.is_none() {
                                                                    theme.border
                                                                } else {
                                                                    theme.accent
                                                                },
                                                            )
                                                            .flex()
                                                            .flex_row()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .child(input_icon_button("file", &theme))
                                                            .child(input_icon_button("mode", &theme))
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .min_h(px(28.0))
                                                                    .px(px(4.0))
                                                                    .py(px(4.0))
                                                                    .text_sm()
                                                                    .cursor_text()
                                                                    .track_focus(&self.focus_handle)
                                                                    .on_mouse_down(
                                                                        MouseButton::Left,
                                                                        {
                                                                            let focus =
                                                                                self.focus_handle
                                                                                    .clone();
                                                                            move |_, window, cx| {
                                                                                focus.focus(
                                                                                    window, cx,
                                                                                );
                                                                                cx.stop_propagation();
                                                                            }
                                                                        },
                                                                    )
                                                                    .child(ComposerInput {
                                                                        entity: cx.entity(),
                                                                        focus_handle: self
                                                                            .focus_handle
                                                                            .clone(),
                                                                        text: self
                                                                            .input_text
                                                                            .clone(),
                                                                        marked_range: self
                                                                            .marked_range
                                                                            .clone(),
                                                                        placeholder: "Ask z-claw"
                                                                            .into(),
                                                                        theme,
                                                                    }),
                                                            ),
                                                    )
                                                    .child(
                                                        Button::new("Send")
                                                            .variant(if can_send {
                                                                ButtonVariant::Primary
                                                            } else {
                                                                ButtonVariant::Secondary
                                                            })
                                                            .disabled(!can_send)
                                                            .on_click({
                                                                let entity = cx.entity();
                                                                move |_, window, cx| {
                                                                    entity.update(
                                                                        cx,
                                                                        |this, cx| {
                                                                            this.submit(
                                                                                window, cx,
                                                                            )
                                                                        },
                                                                    );
                                                                    cx.stop_propagation();
                                                                }
                                                            }),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap(px(14.0))
                                                    .pl(px(4.0))
                                                    .text_size(px(10.0))
                                                    .text_color(theme.text_subtle)
                                                    .child("Ctrl+Enter send")
                                                    .child("Esc cancel")
                                                    .child("Tools require approval"),
                                            ),
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
                    .on_save({
                        let entity = cx.entity();
                        move |settings, cx| {
                            entity.update(cx, |this: &mut MainWindow, cx| {
                                this.current_settings = settings;
                                this.show_settings = false;
                                this.editing_field = None;
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

fn toolbar_pill(label: &'static str, theme: &ThemeColors) -> impl IntoElement {
    div()
        .h(px(26.0))
        .px(px(9.0))
        .flex()
        .items_center()
        .rounded_sm()
        .border_1()
        .border_color(theme.border)
        .bg(theme.sidebar_bg)
        .text_size(px(11.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.text_muted)
        .hover(|el| el.border_color(theme.border_strong).text_color(theme.text))
        .child(label)
}

fn input_icon_button(label: &'static str, theme: &ThemeColors) -> impl IntoElement {
    div()
        .w(px(26.0))
        .h(px(26.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_sm()
        .text_size(px(10.0))
        .text_color(theme.text_muted)
        .hover(|el| el.bg(theme.overlay).text_color(theme.text))
        .child(label)
}

struct ComposerInput {
    entity: Entity<MainWindow>,
    focus_handle: FocusHandle,
    text: SharedString,
    marked_range: Option<Range<usize>>,
    placeholder: SharedString,
    theme: ThemeColors,
}

struct ComposerPrepaint {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
}

impl IntoElement for ComposerInput {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ComposerInput {
    type RequestLayoutState = ();
    type PrepaintState = ComposerPrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let style = window.text_style();
        let is_empty = self.text.is_empty();
        let display_text = if is_empty {
            self.placeholder.clone()
        } else {
            self.text.clone()
        };
        let text_color = if is_empty {
            self.theme.text_subtle
        } else {
            self.theme.text
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color.into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if !is_empty {
            if let Some(marked_range) = self.marked_range.as_ref() {
                vec![
                    TextRun {
                        len: marked_range.start,
                        ..run.clone()
                    },
                    TextRun {
                        len: marked_range.end - marked_range.start,
                        underline: Some(UnderlineStyle {
                            color: Some(self.theme.accent.into()),
                            thickness: px(1.0),
                            wavy: false,
                        }),
                        ..run.clone()
                    },
                    TextRun {
                        len: display_text.len() - marked_range.end,
                        ..run
                    },
                ]
                .into_iter()
                .filter(|run| run.len > 0)
                .collect::<Vec<_>>()
            } else {
                vec![run]
            }
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);
        let cursor_pos = line.x_for_index(self.text.len());
        let cursor = if !is_empty && self.focus_handle.is_focused(window) {
            Some(fill(
                Bounds::new(
                    point(bounds.left() + cursor_pos, bounds.top()),
                    size(px(1.5), bounds.bottom() - bounds.top()),
                ),
                self.theme.accent,
            ))
        } else {
            None
        };

        ComposerPrepaint {
            line: Some(line),
            cursor,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.handle_input(
            &self.focus_handle,
            ElementInputHandler::new(bounds, self.entity.clone()),
            cx,
        );

        if let Some(line) = prepaint.line.take() {
            let _ = line.paint(
                bounds.origin,
                window.line_height(),
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            );
        }

        if let Some(cursor) = prepaint.cursor.take() {
            window.paint_quad(cursor);
        }
    }
}

// ── Text input ──

impl MainWindow {
    /// Apply text edit to chat input, handling insert and delete via range.
    fn apply_chat_edit(&mut self, range_utf16: Option<Range<usize>>, text: &str) -> Range<usize> {
        let mut current = self.input_text.to_string();
        let has_explicit_range = range_utf16.is_some();
        let has_marked_range = self.marked_range.is_some();
        let range = range_utf16
            .map(|range| utf16_range_to_byte_range(&current, range))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| current.len()..current.len());

        let cursor =
            if text.is_empty() && range.is_empty() && !has_explicit_range && !has_marked_range {
                current.pop();
                current.len()
            } else {
                current.replace_range(range.clone(), text);
                range.start + text.len()
            };
        self.input_text = current.into();
        cursor..cursor
    }

    fn active_text_utf16_len(&self) -> usize {
        let value = if let Some(ref field) = self.editing_field {
            match field {
                SettingsField::Model => &self.current_settings.model,
                SettingsField::Endpoint => &self.current_settings.endpoint,
                SettingsField::ApiKey => &self.current_settings.api_key,
            }
        } else {
            self.input_text.as_ref()
        };
        value.encode_utf16().count()
    }

    fn byte_range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        byte_range_to_utf16_range(&self.input_text, range)
    }
}

fn utf16_range_to_byte_range(text: &str, range: Range<usize>) -> Range<usize> {
    utf16_index_to_byte_index(text, range.start)..utf16_index_to_byte_index(text, range.end)
}

fn utf16_index_to_byte_index(text: &str, utf16_index: usize) -> usize {
    let mut units = 0;
    for (byte_index, ch) in text.char_indices() {
        if units >= utf16_index {
            return byte_index;
        }
        units += ch.len_utf16();
    }
    text.len()
}

fn byte_index_to_utf16_index(text: &str, byte_index: usize) -> usize {
    let mut units = 0;
    for (idx, ch) in text.char_indices() {
        if idx >= byte_index {
            return units;
        }
        units += ch.len_utf16();
    }
    units
}

fn byte_range_to_utf16_range(text: &str, range: &Range<usize>) -> Range<usize> {
    byte_index_to_utf16_index(text, range.start)..byte_index_to_utf16_index(text, range.end)
}

impl EntityInputHandler for MainWindow {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
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
            let text = self.input_text.to_string();
            let byte_range = utf16_range_to_byte_range(&text, range);
            *adjusted_range = Some(byte_range_to_utf16_range(&text, &byte_range));
            Some(text[byte_range].to_string())
        }
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let len = self.active_text_utf16_len();
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
        if self.editing_field.is_some() {
            None
        } else {
            self.marked_range
                .as_ref()
                .map(|range| self.byte_range_to_utf16(range))
        }
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.marked_range = None;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editing_field.is_some() {
            if text == "\r" || text == "\n" || text == "\r\n" {
                self.editing_field = None;
                cx.notify();
                return;
            }
            if text.is_empty() {
                self.backspace_field(cx);
            } else {
                self.apply_to_field(text, cx);
            }
            return;
        }

        // Editing chat input
        if text == "\n" || text == "\r" || text == "\r\n" {
            self.marked_range = None;
            self.submit(window, cx);
            return;
        }
        self.apply_chat_edit(range, text);
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
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
        let inserted_range = self.apply_chat_edit(range, new_text);
        self.marked_range = if new_text.is_empty() {
            None
        } else {
            Some(inserted_range.start - new_text.len()..inserted_range.start)
        };
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        if range_utf16.start <= self.active_text_utf16_len() {
            Some(element_bounds)
        } else {
            None
        }
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.active_text_utf16_len())
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
