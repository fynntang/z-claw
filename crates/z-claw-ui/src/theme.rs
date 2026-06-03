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
            // Dark neutral console scale with restrained Catppuccin accents.
            background: rgb(0x11111b),
            sidebar_bg: rgb(0x181825),
            surface: rgb(0x1e1e2e),
            overlay: rgb(0x313244),
            input_bg: rgb(0x181825),

            text: rgb(0xcdd6f4),
            text_muted: rgb(0xa6adc8),
            text_subtle: rgb(0x6c7086),

            // Borders
            border: rgb(0x313244),

            accent: rgb(0x89b4fa),
            error: rgb(0xf38ba8),
            success: rgb(0xa6e3a1),
            warning: rgb(0xf9e2af),
        }
    }
}

impl Global for ThemeColors {}

/// Convenience accessor for the current theme colors.
pub fn theme_colors(cx: &App) -> ThemeColors {
    *cx.global::<ThemeColors>()
}
