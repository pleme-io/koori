//! Koori (氷) — cold-toned colorscheme for Neovim inspired by arctic palettes.
//!
//! Part of the blnvim-ng distribution — a Rust-native Neovim plugin suite.
//! Built with [`nvim-oxi`](https://github.com/noib3/nvim-oxi) for zero-cost
//! Neovim API bindings and [`tane`] for high-level highlight management.
//!
//! # Usage
//!
//! Load the plugin in Neovim and run `:KooriApply` to activate the theme.
//!
//! # Customisation
//!
//! Set global variables before calling `:KooriApply`:
//!
//! ```lua
//! vim.g.koori_transparent = true  -- transparent background
//! vim.g.koori_style = "flat"      -- disable italic styles
//! vim.g.koori_bg = "#000000"      -- custom background colour
//! ```

pub mod highlight;
pub mod palettes;
pub mod theme;

use nvim_oxi as oxi;
use tane::prelude::*;

#[oxi::plugin]
fn koori() -> oxi::Result<()> {
    UserCommand::new("KooriApply")
        .desc("Apply the Koori colorscheme")
        .bar()
        .register(|_args| {
            let theme = highlight::build_theme_from_config()
                .map_err(|e| tane::Error::Custom(e.to_string()))?;
            highlight::apply_theme(&theme)
                .map_err(|e| tane::Error::Custom(e.to_string()))?;
            Ok(())
        })
        .map_err(|e| oxi::Error::from(oxi::api::Error::Other(e.to_string())))?;

    Ok(())
}
