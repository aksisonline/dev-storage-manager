use gpui::Hsla;

/// Full Black Coder theme - minimal, boxy, snappy
#[allow(dead_code)]
pub struct Theme {
    // Border colors
    pub border: Hsla,
    pub border_focused: Hsla,
    pub border_disabled: Hsla,

    // Background colors
    pub background: Hsla,
    pub surface: Hsla,
    pub element_bg: Hsla,

    // Text colors
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_dim: Hsla,
    pub text_accent: Hsla,

    // Status colors
    pub error: Hsla,
    pub warning: Hsla,
    pub success: Hsla,
    pub info: Hsla,
}

impl Theme {
    pub fn coder_black() -> Self {
        Self {
            // Border colors - sharp, defined
            border: rgb(0x333333),
            border_focused: rgb(0x666666),
            border_disabled: rgb(0x1a1a1a),

            // Background colors - pure black theme
            background: rgb(0x000000),
            surface: rgb(0x0a0a0a),
            element_bg: rgb(0x1a1a1a),

            // Text colors - high contrast
            text: rgb(0xffffff),
            text_muted: rgb(0x999999),
            text_dim: rgb(0x666666),
            text_accent: rgb(0x00ff88), // Bright cyan-green

            // Status colors - vivid
            error: rgb(0xff4444),
            warning: rgb(0xffaa00),
            success: rgb(0x00ff88),
            info: rgb(0x00aaff),
        }
    }
}

fn rgb(hex: u32) -> Hsla {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;

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
        a: 1.0,
    }
}
