//! Media-slot framing shared by `ItemMedia` and `EmptyMedia`.
//!
//! A media [`Frame`] combines a container [`Spec`] with the size and clipping of the tile.

use std::ops::Add;

use super::container::Spec;
use super::tweak::Tweak;

/// How a media slot frames its content: its paint, an optional square tile
/// size and whether the tile clips its content.
#[derive(Debug, Clone)]
pub struct Frame {
    paint: Spec,
    tile: Option<f32>,
    clip: bool,
}

impl Frame {
    pub(crate) fn new(paint: Spec, tile: Option<f32>, clip: bool) -> Self {
        Self { paint, tile, clip }
    }

    /// The container style painted behind the content.
    pub fn paint(&self) -> Spec {
        self.paint.clone()
    }

    /// The side of the square tile, in logical pixels, when the variant
    /// fixes one.
    pub fn tile(&self) -> Option<f32> {
        self.tile
    }

    /// Whether the tile clips its content (`overflow-hidden`).
    pub fn clip(&self) -> bool {
        self.clip
    }
}

impl<T: Into<Tweak>> Add<T> for Frame {
    type Output = Frame;

    fn add(mut self, rhs: T) -> Frame {
        self.paint = self.paint + rhs;
        self
    }
}
