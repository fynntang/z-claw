use gpui::{App, Global, Rgba, rgb};

/// Catppuccin Mocha semantic color palette.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    // Backgrounds
    pub background: Rgba,
    pub sidebar_bg: Rgba,
    pub surface: Rgba,
    pub overlay: Rgba,
    pub input_bg: Rgba,

    // Text
    pub text: Rgba,
    pub text_muted: Rgba,
    pub text_subtle: Rgba,

    // Borders
    pub border: Rgba,

    // Accents
    pub accent: Rgba,
    pub error: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl ThemeColors {
    pub fn catppuccin_mocha() -> Self {
        Self {
            // Backgrounds — Catppuccin Mocha neutral scale
            background: rgb(0x1e1e2e), // Base
            sidebar_bg: rgb(0x181825), // Mantle
            surface: rgb(0x313244),    // Surface0
            overlay: rgb(0x45475a),    // Surface1
            input_bg: rgb(0x1e1e2e),   // Base

            // Text — Catppuccin Mocha text scale
            text: rgb(0xcdd6f4),        // Text
            text_muted: rgb(0xa6adc8),  // Subtext0
            text_subtle: rgb(0x585b70), // Surface2

            // Borders
            border: rgb(0x313244), // Surface0

            // Accents — Catppuccin Mocha accent colors
            accent: rgb(0x89b4fa),  // Blue
            error: rgb(0xf38ba8),   // Red
            success: rgb(0xa6e3a1), // Green
            warning: rgb(0xf9e2af), // Yellow
        }
    }
}

impl Global for ThemeColors {}

/// Convenience accessor for the current theme colors.
pub fn theme_colors(cx: &App) -> ThemeColors {
    *cx.global::<ThemeColors>()
}
