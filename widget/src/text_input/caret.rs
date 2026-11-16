//! Where the caret goes between the graphemes of a laid out paragraph.
use iced_core::text::Paragraph as _;
use iced_graphics::text::Paragraph;
use iced_graphics::text::cosmic_text;

/// A paragraph that places a caret from the layout it already has, without
/// shaping its text again.
pub trait Caret {
    /// The distance from the start of the paragraph's first line to the
    /// caret before the byte `offset` of its text.
    fn caret_x(&self, offset: usize) -> f32;
}

impl Caret for Paragraph {
    fn caret_x(&self, offset: usize) -> f32 {
        let buffer = self.buffer();
        // The buffer is laid out in hinted pixels.
        let size = self.size().0;
        let scale = if size > 0.0 {
            buffer.metrics().font_size / size
        } else {
            1.0
        };

        buffer
            .cursor_position(&cosmic_text::Cursor::new(0, offset))
            .map_or(0.0, |(x, _)| x / scale)
    }
}

/// The paragraph of the null renderer has no layout.
impl Caret for () {
    fn caret_x(&self, _offset: usize) -> f32 {
        0.0
    }
}
