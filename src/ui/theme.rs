use gpui::Hsla;

/// Zen Dark theme colors extracted from theme.json
#[allow(dead_code)]
pub struct Theme {
    // Border colors
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,
    pub border_disabled: Hsla,

    // Background colors
    pub background: Hsla,
    pub elevated_surface_background: Hsla,
    pub surface_background: Hsla,
    pub panel_background: Hsla,
    pub editor_background: Hsla,

    // Element colors
    pub element_background: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,

    // Text colors
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,

    // Icon colors
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_accent: Hsla,

    // Status colors
    pub error: Hsla,
    pub error_background: Hsla,
    pub error_border: Hsla,
    pub warning: Hsla,
    pub warning_background: Hsla,
    pub success: Hsla,
    pub success_background: Hsla,
    pub info: Hsla,
    pub info_background: Hsla,

    // Scrollbar colors
    pub scrollbar_thumb_background: Hsla,
    pub scrollbar_thumb_hover_background: Hsla,
    pub scrollbar_track_background: Hsla,
}

impl Theme {
    pub fn zen_dark() -> Self {
        Self {
            // Border colors
            border: parse_hex("#262626ff"),
            border_variant: parse_hex("#171717ff"),
            border_focused: parse_hex("#737373ff"),
            border_selected: parse_hex("#404040ff"),
            border_disabled: parse_hex("#262626ff"),

            // Background colors
            background: parse_hex("#171717ff"),
            elevated_surface_background: parse_hex("#171717ff"),
            surface_background: parse_hex("#171717ff"),
            panel_background: parse_hex("#171717ff"),
            editor_background: parse_hex("#0a0a0aff"),

            // Element colors
            element_background: parse_hex("#171717ff"),
            element_hover: parse_hex("#262626ff"),
            element_active: parse_hex("#262626ff"),
            element_selected: parse_hex("#262626ff"),
            element_disabled: parse_hex("#171717ff"),

            // Text colors
            text: parse_hex("#e5e5e5ff"),
            text_muted: parse_hex("#a3a3a3ff"),
            text_placeholder: parse_hex("#737373ff"),
            text_disabled: parse_hex("#737373ff"),
            text_accent: parse_hex("#14b8a6ff"),

            // Icon colors
            icon: parse_hex("#e5e5e5ff"),
            icon_muted: parse_hex("#a3a3a3ff"),
            icon_disabled: parse_hex("#737373ff"),
            icon_accent: parse_hex("#14b8a6ff"),

            // Status colors
            error: parse_hex("#f87171ff"),
            error_background: parse_hex("#f8717133"),
            error_border: parse_hex("#f87171ff"),
            warning: parse_hex("#fbbf24ff"),
            warning_background: parse_hex("#fbbf2433"),
            success: parse_hex("#4ade80ff"),
            success_background: parse_hex("#4ade8033"),
            info: parse_hex("#60a5faff"),
            info_background: parse_hex("#60a5fa33"),

            // Scrollbar colors
            scrollbar_thumb_background: parse_hex("#52525233"),
            scrollbar_thumb_hover_background: parse_hex("#262626ff"),
            scrollbar_track_background: parse_hex("#171717ff"),
        }
    }
}

/// Parse a hex color string (with optional alpha) into HSLA
fn parse_hex(hex: &str) -> Hsla {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };

    // Convert RGB to HSLA
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let a = a as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    Hsla {
        h: h / 360.0,
        s,
        l,
        a,
    }
}
