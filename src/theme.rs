//! Theme definition types.
//!
//! A theme in Koori is a [`Palette`] of raw colours plus a [`ThemeColors`]
//! mapping that assigns semantic meaning to each palette slot. The
//! [`ThemeColors`] is what [`crate::highlight`] consumes when applying
//! highlight groups.

/// Raw colour palette — every value is a CSS hex colour (`#rrggbb`).
#[derive(Debug, Clone)]
pub struct Palette {
    // Backgrounds
    pub bg0: String,
    pub bg1: String,
    pub bg2: String,
    pub bg3: String,

    // Foregrounds
    pub fg0: String,
    pub fg1: String,
    pub fg2: String,
    pub fg3: String,

    // Accent
    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub green: String,
    pub cyan: String,
    pub blue: String,
    pub purple: String,
    pub magenta: String,

    // Semantic
    pub error: String,
    pub warn: String,
    pub info: String,
    pub hint: String,

    // Git
    pub git_add: String,
    pub git_change: String,
    pub git_delete: String,

    // Selection / visual
    pub selection: String,
    pub cursor_line: String,
    pub comment: String,

    // Border / float
    pub border: String,
    pub float_bg: String,
    pub float_border: String,
}

impl Palette {
    /// Return every colour as `(name, value)` pairs — useful for validation.
    #[must_use]
    pub fn all_colors(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("bg0", &self.bg0),
            ("bg1", &self.bg1),
            ("bg2", &self.bg2),
            ("bg3", &self.bg3),
            ("fg0", &self.fg0),
            ("fg1", &self.fg1),
            ("fg2", &self.fg2),
            ("fg3", &self.fg3),
            ("red", &self.red),
            ("orange", &self.orange),
            ("yellow", &self.yellow),
            ("green", &self.green),
            ("cyan", &self.cyan),
            ("blue", &self.blue),
            ("purple", &self.purple),
            ("magenta", &self.magenta),
            ("error", &self.error),
            ("warn", &self.warn),
            ("info", &self.info),
            ("hint", &self.hint),
            ("git_add", &self.git_add),
            ("git_change", &self.git_change),
            ("git_delete", &self.git_delete),
            ("selection", &self.selection),
            ("cursor_line", &self.cursor_line),
            ("comment", &self.comment),
            ("border", &self.border),
            ("float_bg", &self.float_bg),
            ("float_border", &self.float_border),
        ]
    }
}

/// Semantic mapping from a [`Palette`] into the colour slots consumed by
/// [`crate::highlight::apply_theme`].
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // ── Editor ──────────────────────────────────────────────────────
    pub editor_bg: String,
    pub editor_fg: String,
    pub cursor_line_bg: String,
    pub line_nr_fg: String,
    pub line_nr_active_fg: String,
    pub sign_column_bg: String,
    pub visual_bg: String,
    pub search_bg: String,
    pub search_fg: String,
    pub inc_search_bg: String,
    pub inc_search_fg: String,
    pub match_paren_bg: String,
    pub match_paren_fg: String,
    pub pmenu_bg: String,
    pub pmenu_fg: String,
    pub pmenu_sel_bg: String,
    pub pmenu_sel_fg: String,
    pub pmenu_sbar_bg: String,
    pub pmenu_thumb_bg: String,
    pub status_line_bg: String,
    pub status_line_fg: String,
    pub tab_line_bg: String,
    pub tab_line_fg: String,
    pub tab_line_sel_bg: String,
    pub tab_line_sel_fg: String,
    pub win_separator_fg: String,
    pub non_text_fg: String,
    pub folded_bg: String,
    pub folded_fg: String,
    pub float_bg: String,
    pub float_border_fg: String,
    pub title_fg: String,

    // ── Syntax ──────────────────────────────────────────────────────
    pub comment_fg: String,
    pub comment_style: Style,
    pub constant_fg: String,
    pub string_fg: String,
    pub character_fg: String,
    pub number_fg: String,
    pub boolean_fg: String,
    pub identifier_fg: String,
    pub function_fg: String,
    pub statement_fg: String,
    pub keyword_fg: String,
    pub keyword_style: Style,
    pub operator_fg: String,
    pub preproc_fg: String,
    pub type_fg: String,
    pub special_fg: String,
    pub delimiter_fg: String,
    pub tag_fg: String,

    // ── Diagnostics ─────────────────────────────────────────────────
    pub diag_error_fg: String,
    pub diag_warn_fg: String,
    pub diag_info_fg: String,
    pub diag_hint_fg: String,

    // ── Git / Diff ──────────────────────────────────────────────────
    pub diff_add_fg: String,
    pub diff_change_fg: String,
    pub diff_delete_fg: String,
    pub diff_text_bg: String,
}

/// Text style modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Normal,
    Italic,
    Bold,
    BoldItalic,
}

impl ThemeColors {
    /// Derive a full theme mapping from a raw palette.
    #[must_use]
    pub fn from_palette(p: &Palette) -> Self {
        Self {
            // Editor
            editor_bg: p.bg0.clone(),
            editor_fg: p.fg0.clone(),
            cursor_line_bg: p.cursor_line.clone(),
            line_nr_fg: p.fg3.clone(),
            line_nr_active_fg: p.fg1.clone(),
            sign_column_bg: p.bg0.clone(),
            visual_bg: p.selection.clone(),
            search_bg: p.yellow.clone(),
            search_fg: p.bg0.clone(),
            inc_search_bg: p.orange.clone(),
            inc_search_fg: p.bg0.clone(),
            match_paren_bg: p.bg3.clone(),
            match_paren_fg: p.yellow.clone(),
            pmenu_bg: p.bg2.clone(),
            pmenu_fg: p.fg0.clone(),
            pmenu_sel_bg: p.blue.clone(),
            pmenu_sel_fg: p.bg0.clone(),
            pmenu_sbar_bg: p.bg3.clone(),
            pmenu_thumb_bg: p.fg3.clone(),
            status_line_bg: p.bg2.clone(),
            status_line_fg: p.fg1.clone(),
            tab_line_bg: p.bg1.clone(),
            tab_line_fg: p.fg2.clone(),
            tab_line_sel_bg: p.bg0.clone(),
            tab_line_sel_fg: p.fg0.clone(),
            win_separator_fg: p.border.clone(),
            non_text_fg: p.fg3.clone(),
            folded_bg: p.bg2.clone(),
            folded_fg: p.fg2.clone(),
            float_bg: p.float_bg.clone(),
            float_border_fg: p.float_border.clone(),
            title_fg: p.blue.clone(),

            // Syntax
            comment_fg: p.comment.clone(),
            comment_style: Style::Italic,
            constant_fg: p.orange.clone(),
            string_fg: p.green.clone(),
            character_fg: p.green.clone(),
            number_fg: p.orange.clone(),
            boolean_fg: p.orange.clone(),
            identifier_fg: p.fg0.clone(),
            function_fg: p.blue.clone(),
            statement_fg: p.purple.clone(),
            keyword_fg: p.purple.clone(),
            keyword_style: Style::Italic,
            operator_fg: p.cyan.clone(),
            preproc_fg: p.magenta.clone(),
            type_fg: p.yellow.clone(),
            special_fg: p.cyan.clone(),
            delimiter_fg: p.fg2.clone(),
            tag_fg: p.red.clone(),

            // Diagnostics
            diag_error_fg: p.error.clone(),
            diag_warn_fg: p.warn.clone(),
            diag_info_fg: p.info.clone(),
            diag_hint_fg: p.hint.clone(),

            // Git / Diff
            diff_add_fg: p.git_add.clone(),
            diff_change_fg: p.git_change.clone(),
            diff_delete_fg: p.git_delete.clone(),
            diff_text_bg: p.bg3.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palettes::frost;

    #[test]
    fn theme_colors_from_frost_palette() {
        let palette = frost::palette();
        let theme = ThemeColors::from_palette(&palette);

        assert_eq!(theme.editor_bg, palette.bg0);
        assert_eq!(theme.editor_fg, palette.fg0);
        assert_eq!(theme.diag_error_fg, palette.error);
        assert_eq!(theme.diff_add_fg, palette.git_add);
        assert_eq!(theme.comment_fg, palette.comment);
    }

    #[test]
    fn theme_colors_default_styles() {
        let palette = frost::palette();
        let theme = ThemeColors::from_palette(&palette);

        assert_eq!(theme.comment_style, Style::Italic);
        assert_eq!(theme.keyword_style, Style::Italic);
    }

    #[test]
    fn theme_colors_search_contrasts_with_bg() {
        let palette = frost::palette();
        let theme = ThemeColors::from_palette(&palette);

        // Search foreground should be the dark background for contrast
        assert_eq!(theme.search_fg, palette.bg0);
        assert_eq!(theme.inc_search_fg, palette.bg0);
    }

    #[test]
    fn theme_colors_syntax_uses_distinct_colors() {
        let palette = frost::palette();
        let theme = ThemeColors::from_palette(&palette);

        // Key syntax elements should use different colours
        let syntax_colors = [
            &theme.string_fg,
            &theme.function_fg,
            &theme.keyword_fg,
            &theme.type_fg,
            &theme.operator_fg,
            &theme.constant_fg,
        ];
        for (i, a) in syntax_colors.iter().enumerate() {
            for (j, b) in syntax_colors.iter().enumerate() {
                if i != j {
                    assert_ne!(
                        a, b,
                        "syntax colors at indices {i} and {j} should differ",
                    );
                }
            }
        }
    }

    #[test]
    fn style_debug_representation() {
        assert_eq!(format!("{:?}", Style::Normal), "Normal");
        assert_eq!(format!("{:?}", Style::Italic), "Italic");
        assert_eq!(format!("{:?}", Style::Bold), "Bold");
        assert_eq!(format!("{:?}", Style::BoldItalic), "BoldItalic");
    }

    #[test]
    fn palette_clone_is_equal() {
        let p = frost::palette();
        let p2 = p.clone();
        // Verify clone produces identical values
        for ((name_a, val_a), (name_b, val_b)) in
            p.all_colors().iter().zip(p2.all_colors().iter())
        {
            assert_eq!(name_a, name_b);
            assert_eq!(val_a, val_b);
        }
    }

    #[test]
    fn theme_colors_pmenu_selection_is_prominent() {
        let palette = frost::palette();
        let theme = ThemeColors::from_palette(&palette);

        // Selected item should stand out from the menu background
        assert_ne!(theme.pmenu_sel_bg, theme.pmenu_bg);
        assert_ne!(theme.pmenu_sel_fg, theme.pmenu_fg);
    }
}
