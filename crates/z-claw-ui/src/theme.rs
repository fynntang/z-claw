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
    pub border_strong: Rgba,

    // Accents
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub accent_subtle: Rgba,
    pub accent_text: Rgba,
    pub error: Rgba,
    pub error_bg: Rgba,
    pub success: Rgba,
    pub success_bg: Rgba,
    pub warning: Rgba,
    pub warning_bg: Rgba,
    pub info: Rgba,
    pub info_bg: Rgba,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl ThemeColors {
    pub fn catppuccin_mocha() -> Self {
        Self {
            // Warm dark workbench scale inspired by the local prototype.
            background: rgb(0x18191d),
            sidebar_bg: rgb(0x141519),
            surface: rgb(0x1c1d22),
            overlay: rgb(0x24252c),
            input_bg: rgb(0x141519),

            text: rgb(0xc9cad1),
            text_muted: rgb(0x8b8d94),
            text_subtle: rgb(0x5e6068),

            border: rgb(0x2a2b31),
            border_strong: rgb(0x3a3b42),

            accent: rgb(0x5b8cf0),
            accent_hover: rgb(0x7aa5f5),
            accent_subtle: rgb(0x1a2540),
            accent_text: rgb(0x8ab4f8),
            error: rgb(0xf85149),
            error_bg: rgb(0x2b1518),
            success: rgb(0x3fb950),
            success_bg: rgb(0x162b1a),
            warning: rgb(0xd29922),
            warning_bg: rgb(0x2b2410),
            info: rgb(0x58a6ff),
            info_bg: rgb(0x0d2336),
        }
    }
}

impl Global for ThemeColors {}

/// Convenience accessor for the current theme colors.
pub fn theme_colors(cx: &App) -> ThemeColors {
    *cx.global::<ThemeColors>()
}
