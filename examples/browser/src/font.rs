//! Bundled monospace font for inline and fenced code rendering.
//!
//! Ships [Geist Mono] (1.8.0, SIL OFL 1.1) for consistent code rendering.
//!
//! Wired into the iced application via `.font(font::GEIST_MONO)` in
//! `main.rs`, and consumed by `page::inline_style_from_palette` to
//! drive both `inline_code_font` and `code_block_font` on the
//! markdown style.
//!
//! License attribution lives next to the TTF at
//! `examples/browser/fonts/GeistMono-OFL.txt`.
//!
//! [Geist Mono]: https://github.com/vercel/geist-font

/// Embedded TTF bytes for Geist Mono Regular. Pass directly to iced's
/// `Application::font(...)` builder so cosmic-text registers the family
/// at startup; afterwards any [`iced::Font`] whose family matches
/// [`GEIST_MONO_NAME`] resolves against these bytes.
pub const GEIST_MONO: &[u8] = include_bytes!("../fonts/GeistMono-Regular.ttf");

/// Family name embedded in the bundled TTF (verified via the font's
/// `name` table — entry "Geist Mono Regular" splits into family "Geist
/// Mono" + style "Regular"). Use with [`iced::Font::new`] when assigning
/// the font to a widget or markdown style field.
pub const GEIST_MONO_NAME: &str = "Geist Mono";
