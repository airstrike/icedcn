//! Bundled fonts: Geist Mono for code and emoji. The symbols Geist lacks come
//! from [`icedcn::font`].
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
    use iced_widget::renderer::graphics::text::cosmic_text::skrifa::{FontRef, MetadataProvider};
    use iced_widget::renderer::graphics::text::cosmic_text::{
        Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, SwashContent, Weight,
        fontdb,
    };

    use iced_widget::renderer::graphics::text::font_system;
    use icedcn::View;
    use icedcn::font::{SYMBOLS, SYMBOLS_NAME};
    use icedcn::widget::text;

    use super::{BODY, EMOJI, GEIST_MONO, GEIST_MONO_NAME};

    /// Every symbol the browser and the widgets show, or that a shortcut label
    /// can spell: the Mac modifier and key symbols, arrows, marks, typography
    /// and the box drawing of the composition snippets.
    const INVENTORY: &str =
        "⌘⌥⇧⌃⌄⌅⇪⌫⌦↩↵⏎⇥⇤⎋⇞⇟⏏␣↑↓←→↖↗↘↙↔↕↺↻⇐⇒⇔✓✗×÷−±≠≤≥≈∞∅…•·◦‹›«»‘’“”–—■□▪◆◇▲▼◀▶●○★☆─│└├┌┐┘┤┬┴┼╭╮╯╰";

    /// The fonts a web build has, which has no system fonts to fall back on.
    fn web() -> FontSystem {
        let mut database = fontdb::Database::new();
        database.load_font_data(crate::icon::FONT.to_vec());
        database.load_font_data(GEIST_MONO.to_vec());
        database.load_font_data(SYMBOLS.to_vec());
        database.load_font_data(include_bytes!("../fonts/Geist.ttf").to_vec());
        database.load_font_data(EMOJI.to_vec());
        database.set_monospace_family(GEIST_MONO_NAME);
        FontSystem::new_with_locale_and_db("en-US".into(), database)
    }

    #[test]
    fn the_widgets_load_the_symbols_without_any_setup() {
        let _: View<'_> = text::muted("⌘K").into();
        let fonts = font_system().read().expect("font system");
        assert!(fonts.families().any(|family| family == SYMBOLS_NAME));
    }

    #[test]
    fn symbols_cover_what_geist_lacks() {
        let charmap = |bytes| FontRef::new(bytes).expect("bundled font parses").charmap();
        let geist = charmap(include_bytes!("../fonts/Geist.ttf"));
        let symbols = charmap(SYMBOLS);
        let box_drawing = '\u{2500}'..='\u{257F}';
        let missing: String = INVENTORY
            .chars()
            .filter(|c| !box_drawing.contains(c))
            .filter(|&c| geist.map(c).is_none() && symbols.map(c).is_none())
            .collect();
        assert!(missing.is_empty(), "no bundled sans glyph: {missing}");
    }

    #[test]
    fn every_symbol_shapes_from_a_bundled_font_without_system_fonts() {
        let mut fonts = web();
        let geist = FontRef::new(include_bytes!("../fonts/Geist.ttf")).expect("Geist parses");
        let mono = FontRef::new(GEIST_MONO).expect("Geist Mono parses");
        let unbundled = |c| geist.charmap().map(c).is_none() && mono.charmap().map(c).is_none();
        for family in [Family::Name(BODY), Family::Monospace] {
            for weight in [Weight::NORMAL, Weight::MEDIUM, Weight::SEMIBOLD] {
                for symbol in INVENTORY.chars() {
                    let text = format!("{symbol}Z");
                    let mut buffer = Buffer::new(&mut fonts, Metrics::new(14.0, 20.0));
                    buffer.set_text(
                        &text,
                        &Attrs::new().family(family).weight(weight),
                        Shaping::Advanced,
                        None,
                    );
                    buffer.shape_until_scroll(&mut fonts, false);
                    let glyph = buffer
                        .layout_runs()
                        .flat_map(|run| run.glyphs.iter())
                        .find(|glyph| glyph.start == 0)
                        .expect("symbol shapes")
                        .clone();
                    assert_ne!(
                        glyph.glyph_id, 0,
                        "tofu: {symbol} in {family:?} at {weight:?}"
                    );
                    let face = fonts.db().face(glyph.font_id).expect("shaped face");
                    let name = &face.families[0].0;
                    assert!(
                        !face.post_script_name.contains("Emoji"),
                        "emoji for {symbol} in {family:?}: {name}"
                    );
                    if family == Family::Name(BODY) && unbundled(symbol) {
                        assert_eq!(name, SYMBOLS_NAME, "{symbol} at {weight:?}");
                    }
                }
            }
        }
    }

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
