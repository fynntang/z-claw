use std::ops::Range;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;
use gpui_platform::application;
use z_claw_assets::Assets;
use z_claw_ui::views::settings::ProviderSettings;
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

// ── Custom element that registers an EntityInputHandler during paint ──

struct InputElement {
    view: Entity<MainWindow>,
}

impl IntoElement for InputElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
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
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.view.read(cx).focus_handle(cx);
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.view.clone()),
            cx,
        );
    }
}

// ── MainWindow ──

struct MainWindow {
    app_model: Entity<AppModel>,
    input_text: SharedString,
    focus_handle: FocusHandle,
    sessions: Vec<SessionSummary>,
    show_settings: bool,
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
        }
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = std::mem::take(&mut self.input_text);
        if text.trim().is_empty() {
            return;
        }
        self.app_model.update(cx, |model, cx| {
            model.send_text(&text, cx);
        });
        cx.notify();
        self.focus_handle.focus(window, cx);
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app_model.update(cx, |model, cx| model.poll_events(cx));

        // Auto-focus so the InputElement receives keyboard events
        self.focus_handle.clone().focus(window, cx);

        let model = self.app_model.read(cx);
        let theme = *cx.global::<ThemeColors>();
        let messages = model.messages.clone();
        let streaming = model.streaming;
        let pending_approval = model.pending_approval.clone();
        drop(model);

        let sessions = self.sessions.clone();

        let input_text = self.input_text.clone();

        // Sync typed text for send handler
        self.app_model.update(cx, |model, _cx| {
            model.input_text = input_text.to_string();
        });

        let app_model = self.app_model.clone();
        let send_handler: Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + Send + Sync> =
            Arc::new(move |_, _, cx| {
                app_model.update(cx, |model, cx| {
                    model.send_message(cx);
                });
            });

        let app_model_new = self.app_model.clone();
        let input_element = InputElement { view: cx.entity() };
        let show_settings = self.show_settings;

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
            .child(ChatView {
                messages,
                streaming,
                input_text: self.input_text.clone(),
                on_send: Some(send_handler),
            })
            .child(input_element);

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
            let current = ProviderSettings {
                provider_type: "ollama".into(),
                model: "llama3".into(),
                endpoint: "http://localhost:11434/v1".into(),
                api_key: "".into(),
            };
            root = root.child(SettingsPanel::new(current).on_close({
                let entity = cx.entity();
                move |_, cx| {
                    entity.update(cx, |this: &mut MainWindow, cx| {
                        this.show_settings = false;
                        cx.notify();
                    });
                }
            }));
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
        Some(self.input_text.to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let len = self.input_text.len();
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
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if text == "\n" || text == "\r" || text == "\r\n" {
            self.submit(_window, cx);
            return;
        }
        self.input_text = (self.input_text.to_string() + text).into();
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
        self.input_text = (self.input_text.to_string() + new_text).into();
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
        Some(self.input_text.len())
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
