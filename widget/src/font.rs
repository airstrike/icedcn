//! The symbol font the widgets draw with.
//!
//! [`SYMBOLS`] is Inter Symbols, a subset of [Inter] 4.001 with the key and
//! interface symbols most text fonts lack (⌘ ⌥ ⌃ ⌄ ⌫ ⌦ ⏎ ⎋ ⇪ ⇞ ⇟ ⏏ ✓ ✗ and a
//! few shapes), variable in weight and renamed so it never stands in for a
//! full Inter. It is licensed under the SIL Open Font License 1.1; the license
//! ships with the crate at `fonts/InterSymbols-OFL.txt`.
//!
//! The widgets load it into iced's font system before their first text, so a
//! glyph the text's own font lacks falls back to it like to any other loaded
//! font. On the web, which has no system fonts, that is what keeps a shortcut
//! like ⌘K from drawing as a box. Disable the default `symbols` feature to
//! leave it out.
//!
//! [Inter]: https://github.com/rsms/inter

use std::borrow::Cow;
use std::sync::Once;

use iced_graphics::text::font_system;

/// The bytes of Inter Symbols. Passing them to an application's fonts is
/// harmless: iced loads the same bytes once.
pub static SYMBOLS: &[u8] = include_bytes!("../fonts/InterSymbols.ttf");

/// The family name of [`SYMBOLS`].
pub const SYMBOLS_NAME: &str = "Inter Symbols";

/// Loads [`SYMBOLS`] into iced's font system, once.
pub(crate) fn load() {
    static LOAD: Once = Once::new();

    LOAD.call_once(|| {
        if let Ok(mut fonts) = font_system().write() {
            fonts.load_font(Cow::Borrowed(SYMBOLS));
        }
    });
}
