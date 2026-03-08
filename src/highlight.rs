//! Applies a [`ThemeColors`] as Neovim highlight groups via the tane SDK.

use crate::theme::{Style, ThemeColors};
use tane::highlight::Highlight;

/// Helper: apply style modifiers to a [`Highlight`] builder.
fn with_style(hl: Highlight, style: Style) -> Highlight {
    match style {
        Style::Normal => hl,
        Style::Italic => hl.italic(),
        Style::Bold => hl.bold(),
        Style::BoldItalic => hl.bold().italic(),
    }
}

/// Apply all highlight groups defined by `theme` to Neovim.
///
/// Uses `tane::highlight::Highlight` to set each group via
/// `nvim_set_hl(0, ...)`.
pub fn apply_theme(theme: &ThemeColors) -> tane::Result<()> {
    // ── Editor ──────────────────────────────────────────────────────
    Highlight::new("Normal")
        .fg(&theme.editor_fg)
        .bg(&theme.editor_bg)
        .apply()?;
    Highlight::new("NormalFloat")
        .fg(&theme.editor_fg)
        .bg(&theme.float_bg)
        .apply()?;
    Highlight::new("FloatBorder")
        .fg(&theme.float_border_fg)
        .bg(&theme.float_bg)
        .apply()?;
    Highlight::new("FloatTitle")
        .fg(&theme.title_fg)
        .bg(&theme.float_bg)
        .bold()
        .apply()?;
    Highlight::new("CursorLine")
        .bg(&theme.cursor_line_bg)
        .apply()?;
    Highlight::new("CursorColumn")
        .bg(&theme.cursor_line_bg)
        .apply()?;
    Highlight::new("ColorColumn")
        .bg(&theme.cursor_line_bg)
        .apply()?;
    Highlight::new("LineNr")
        .fg(&theme.line_nr_fg)
        .apply()?;
    Highlight::new("CursorLineNr")
        .fg(&theme.line_nr_active_fg)
        .bold()
        .apply()?;
    Highlight::new("SignColumn")
        .bg(&theme.sign_column_bg)
        .apply()?;
    Highlight::new("Visual")
        .bg(&theme.visual_bg)
        .apply()?;
    Highlight::new("VisualNOS")
        .bg(&theme.visual_bg)
        .apply()?;
    Highlight::new("Search")
        .fg(&theme.search_fg)
        .bg(&theme.search_bg)
        .apply()?;
    Highlight::new("IncSearch")
        .fg(&theme.inc_search_fg)
        .bg(&theme.inc_search_bg)
        .apply()?;
    Highlight::new("CurSearch")
        .fg(&theme.inc_search_fg)
        .bg(&theme.inc_search_bg)
        .bold()
        .apply()?;
    Highlight::new("MatchParen")
        .fg(&theme.match_paren_fg)
        .bg(&theme.match_paren_bg)
        .bold()
        .apply()?;
    Highlight::new("NonText")
        .fg(&theme.non_text_fg)
        .apply()?;
    Highlight::new("SpecialKey")
        .fg(&theme.non_text_fg)
        .apply()?;
    Highlight::new("Folded")
        .fg(&theme.folded_fg)
        .bg(&theme.folded_bg)
        .apply()?;
    Highlight::new("FoldColumn")
        .fg(&theme.folded_fg)
        .bg(&theme.sign_column_bg)
        .apply()?;
    Highlight::new("Title")
        .fg(&theme.title_fg)
        .bold()
        .apply()?;

    // Completion menu
    Highlight::new("Pmenu")
        .fg(&theme.pmenu_fg)
        .bg(&theme.pmenu_bg)
        .apply()?;
    Highlight::new("PmenuSel")
        .fg(&theme.pmenu_sel_fg)
        .bg(&theme.pmenu_sel_bg)
        .apply()?;
    Highlight::new("PmenuSbar")
        .bg(&theme.pmenu_sbar_bg)
        .apply()?;
    Highlight::new("PmenuThumb")
        .bg(&theme.pmenu_thumb_bg)
        .apply()?;

    // Status / tab lines
    Highlight::new("StatusLine")
        .fg(&theme.status_line_fg)
        .bg(&theme.status_line_bg)
        .apply()?;
    Highlight::new("StatusLineNC")
        .fg(&theme.non_text_fg)
        .bg(&theme.status_line_bg)
        .apply()?;
    Highlight::new("TabLine")
        .fg(&theme.tab_line_fg)
        .bg(&theme.tab_line_bg)
        .apply()?;
    Highlight::new("TabLineFill")
        .bg(&theme.tab_line_bg)
        .apply()?;
    Highlight::new("TabLineSel")
        .fg(&theme.tab_line_sel_fg)
        .bg(&theme.tab_line_sel_bg)
        .bold()
        .apply()?;
    Highlight::new("WinSeparator")
        .fg(&theme.win_separator_fg)
        .apply()?;
    Highlight::new("VertSplit")
        .link("WinSeparator")
        .apply()?;

    // ── Syntax ──────────────────────────────────────────────────────
    with_style(
        Highlight::new("Comment").fg(&theme.comment_fg),
        theme.comment_style,
    )
    .apply()?;
    Highlight::new("Constant")
        .fg(&theme.constant_fg)
        .apply()?;
    Highlight::new("String")
        .fg(&theme.string_fg)
        .apply()?;
    Highlight::new("Character")
        .fg(&theme.character_fg)
        .apply()?;
    Highlight::new("Number")
        .fg(&theme.number_fg)
        .apply()?;
    Highlight::new("Boolean")
        .fg(&theme.boolean_fg)
        .apply()?;
    Highlight::new("Float")
        .link("Number")
        .apply()?;
    Highlight::new("Identifier")
        .fg(&theme.identifier_fg)
        .apply()?;
    Highlight::new("Function")
        .fg(&theme.function_fg)
        .apply()?;
    Highlight::new("Statement")
        .fg(&theme.statement_fg)
        .apply()?;
    Highlight::new("Conditional")
        .link("Statement")
        .apply()?;
    Highlight::new("Repeat")
        .link("Statement")
        .apply()?;
    Highlight::new("Label")
        .link("Statement")
        .apply()?;
    with_style(
        Highlight::new("Keyword").fg(&theme.keyword_fg),
        theme.keyword_style,
    )
    .apply()?;
    Highlight::new("Operator")
        .fg(&theme.operator_fg)
        .apply()?;
    Highlight::new("Exception")
        .link("Statement")
        .apply()?;
    Highlight::new("PreProc")
        .fg(&theme.preproc_fg)
        .apply()?;
    Highlight::new("Include")
        .link("PreProc")
        .apply()?;
    Highlight::new("Define")
        .link("PreProc")
        .apply()?;
    Highlight::new("Macro")
        .link("PreProc")
        .apply()?;
    Highlight::new("Type")
        .fg(&theme.type_fg)
        .apply()?;
    Highlight::new("StorageClass")
        .link("Type")
        .apply()?;
    Highlight::new("Structure")
        .link("Type")
        .apply()?;
    Highlight::new("Typedef")
        .link("Type")
        .apply()?;
    Highlight::new("Special")
        .fg(&theme.special_fg)
        .apply()?;
    Highlight::new("SpecialChar")
        .link("Special")
        .apply()?;
    Highlight::new("Delimiter")
        .fg(&theme.delimiter_fg)
        .apply()?;
    Highlight::new("Tag")
        .fg(&theme.tag_fg)
        .apply()?;
    Highlight::new("Underlined")
        .fg(&theme.diag_info_fg)
        .underline()
        .apply()?;
    Highlight::new("Error")
        .fg(&theme.diag_error_fg)
        .apply()?;
    Highlight::new("Todo")
        .fg(&theme.diag_info_fg)
        .bold()
        .apply()?;

    // ── Diagnostics ─────────────────────────────────────────────────
    Highlight::new("DiagnosticError")
        .fg(&theme.diag_error_fg)
        .apply()?;
    Highlight::new("DiagnosticWarn")
        .fg(&theme.diag_warn_fg)
        .apply()?;
    Highlight::new("DiagnosticInfo")
        .fg(&theme.diag_info_fg)
        .apply()?;
    Highlight::new("DiagnosticHint")
        .fg(&theme.diag_hint_fg)
        .apply()?;
    Highlight::new("DiagnosticUnderlineError")
        .fg(&theme.diag_error_fg)
        .underline()
        .apply()?;
    Highlight::new("DiagnosticUnderlineWarn")
        .fg(&theme.diag_warn_fg)
        .underline()
        .apply()?;
    Highlight::new("DiagnosticUnderlineInfo")
        .fg(&theme.diag_info_fg)
        .underline()
        .apply()?;
    Highlight::new("DiagnosticUnderlineHint")
        .fg(&theme.diag_hint_fg)
        .underline()
        .apply()?;

    // ── Git / Diff ──────────────────────────────────────────────────
    Highlight::new("DiffAdd")
        .fg(&theme.diff_add_fg)
        .apply()?;
    Highlight::new("DiffChange")
        .fg(&theme.diff_change_fg)
        .apply()?;
    Highlight::new("DiffDelete")
        .fg(&theme.diff_delete_fg)
        .apply()?;
    Highlight::new("DiffText")
        .fg(&theme.diff_change_fg)
        .bg(&theme.diff_text_bg)
        .apply()?;
    Highlight::new("Added")
        .fg(&theme.diff_add_fg)
        .apply()?;
    Highlight::new("Changed")
        .fg(&theme.diff_change_fg)
        .apply()?;
    Highlight::new("Removed")
        .fg(&theme.diff_delete_fg)
        .apply()?;

    Ok(())
}

/// Build a [`ThemeColors`] from the frost palette, optionally applying
/// user overrides from `vim.g.koori_*` variables.
///
/// Supported variables:
/// - `vim.g.koori_bg` — override `editor_bg`
/// - `vim.g.koori_style` — `"default"` (default) or `"flat"` (no italic)
/// - `vim.g.koori_transparent` — `true` to use `NONE` background
/// # Panics
///
/// This function does not panic but returns errors from the Neovim API.
pub fn build_theme_from_config() -> tane::Result<ThemeColors> {
    use tane::config::Config;
    let palette = crate::palettes::frost::palette();
    let mut theme = ThemeColors::from_palette(&palette);

    if let Some(config) = Config::from_global("koori")? {
        if let Some(bg) = config.get_string("bg")? {
            theme.editor_bg = bg;
        }
        if let Some(style) = config.get_string("style")? {
            if style == "flat" {
                theme.comment_style = Style::Normal;
                theme.keyword_style = Style::Normal;
            }
        }
        if let Some(transparent) = config.get_bool("transparent")? {
            if transparent {
                theme.editor_bg = "NONE".into();
                theme.sign_column_bg = "NONE".into();
            }
        }
    }

    Ok(theme)
}

/// Build a [`ThemeColors`] from the frost palette without Neovim
/// interaction (for testing or external use).
#[must_use]
pub fn build_default_theme() -> ThemeColors {
    let palette = crate::palettes::frost::palette();
    ThemeColors::from_palette(&palette)
}
