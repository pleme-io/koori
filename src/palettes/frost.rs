//! Frost — the default dark palette.
//!
//! A cold-toned palette blending Catppuccin Mocha's warmth with
//! Tokyo Night's crisp blues. Designed for long coding sessions.

use crate::theme::Palette;

/// Build the Frost palette.
#[must_use]
pub fn palette() -> Palette {
    Palette {
        // Backgrounds (darkest → lightest)
        bg0: "#1a1b26".into(),
        bg1: "#1e2030".into(),
        bg2: "#24273a".into(),
        bg3: "#2a2e42".into(),

        // Foregrounds (brightest → dimmest)
        fg0: "#cad3f5".into(),
        fg1: "#a5adcb".into(),
        fg2: "#6e738d".into(),
        fg3: "#494d64".into(),

        // Accent colours
        red: "#ed8796".into(),
        orange: "#f5a97f".into(),
        yellow: "#eed49f".into(),
        green: "#a6da95".into(),
        cyan: "#8bd5ca".into(),
        blue: "#8aadf4".into(),
        purple: "#c6a0f6".into(),
        magenta: "#f5bde6".into(),

        // Semantic
        error: "#ed8796".into(),
        warn: "#eed49f".into(),
        info: "#8aadf4".into(),
        hint: "#8bd5ca".into(),

        // Git / diff
        git_add: "#a6da95".into(),
        git_change: "#eed49f".into(),
        git_delete: "#ed8796".into(),

        // Selection / visual
        selection: "#363a4f".into(),
        cursor_line: "#24273a".into(),
        comment: "#5b6078".into(),

        // Border / float
        border: "#494d64".into(),
        float_bg: "#1e2030".into(),
        float_border: "#6e738d".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_colors_are_valid_hex() {
        let p = palette();
        for (name, color) in p.all_colors() {
            assert!(
                color.starts_with('#') && color.len() == 7,
                "palette color `{name}` should be a 7-char hex string, got `{color}`",
            );
            assert!(
                u32::from_str_radix(&color[1..], 16).is_ok(),
                "palette color `{name}` is not valid hex: `{color}`",
            );
        }
    }

    #[test]
    fn palette_has_expected_color_count() {
        let p = palette();
        // 4 bg + 4 fg + 8 accent + 4 semantic + 3 git + 3 selection + 3 border/float = 29
        assert_eq!(p.all_colors().len(), 29);
    }

    #[test]
    fn palette_bg0_is_darkest() {
        let p = palette();
        let luminance = |hex: &str| -> u32 {
            let v = u32::from_str_radix(&hex[1..], 16).unwrap();
            let r = (v >> 16) & 0xff;
            let g = (v >> 8) & 0xff;
            let b = v & 0xff;
            r + g + b
        };
        assert!(
            luminance(&p.bg0) < luminance(&p.bg3),
            "bg0 should be darker than bg3",
        );
    }
}
