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

use iced::advanced::graphics::text::font_system;

/// Sans family shared by the browser controls and documentation prose.
pub const BODY: &str = "Geist";

/// Embedded TTF bytes for Geist Mono Regular. Pass directly to iced's
/// `Application::font(...)` builder so cosmic-text registers the family
/// at startup; afterwards any [`iced::Font`] whose family matches
/// [`GEIST_MONO_NAME`] resolves against these bytes.
pub const GEIST_MONO: &[u8] = include_bytes!("../fonts/GeistMono-Regular.ttf");

/// Family name embedded in the bundled TTF (verified via the font's
/// `name` table — entry "Geist Mono Regular" splits into family "Geist
/// Mono" + style "Regular"). Pass this string to `.font(...)` when assigning
/// the font to a widget or markdown style field.
pub const GEIST_MONO_NAME: &str = "Geist Mono";

/// Resolve the generic monospace family to Geist Mono, so code blocks and
/// inline code keep their default [`iced::Font::MONOSPACE`]. Call it before
/// the application starts, before any text is shaped.
pub fn use_geist_mono() {
    if let Ok(mut fonts) = font_system().write() {
        fonts.raw().db_mut().set_monospace_family(GEIST_MONO_NAME);
    }
}

/// Color emoji fallback for WebAssembly, where system fonts are unavailable.
/// The unmodified COLRv0 font and its license are bundled under `fonts/`.
#[cfg(any(target_arch = "wasm32", test))]
pub const EMOJI: &[u8] = include_bytes!("../fonts/TwemojiMozilla.ttf");

#[cfg(test)]
mod tests {
    use iced_widget::renderer::graphics::text::cosmic_text::{
        Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, SwashContent, fontdb,
    };

    use super::{BODY, EMOJI};

    #[test]
    fn emoji_fallback_shapes_and_rasterizes_in_color_without_system_fonts() {
        let mut database = fontdb::Database::new();
        database.load_font_data(include_bytes!("../fonts/Geist.ttf").to_vec());
        database.load_font_data(EMOJI.to_vec());
        let mut fonts = FontSystem::new_with_locale_and_db("en-US".into(), database);
        let mut cache = SwashCache::new();
        for emoji in ["👍", "👀", "🔥", "😭", "😮", "🚀", "🎉", "👏", "👍🏽", "👩‍💻"]
        {
            let mut buffer = Buffer::new(&mut fonts, Metrics::new(20.0, 28.0));
            buffer.set_text(
                emoji,
                &Attrs::new().family(Family::Name(BODY)),
                Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut fonts, false);
            let glyphs: Vec<_> = buffer
                .layout_runs()
                .flat_map(|run| run.glyphs.iter())
                .collect();
            assert!(!glyphs.is_empty(), "unshaped emoji: {emoji}");
            for glyph in glyphs {
                assert_ne!(glyph.glyph_id, 0, "missing emoji: {emoji}");
                let physical = glyph.physical((0.0, 0.0), 1.0);
                let image = cache
                    .get_image(&mut fonts, physical.cache_key)
                    .as_ref()
                    .expect("emoji rasterizes with the locked renderer");
                assert_eq!(
                    image.content,
                    SwashContent::Color,
                    "monochrome emoji: {emoji}"
                );
                assert!(!image.data.is_empty(), "empty emoji: {emoji}");
            }
        }
    }
}
