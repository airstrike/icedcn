//! Empty-state media styling.
//!
//! Media is unframed by default. The icon style adds a rounded, muted tile.

use std::ops::Add;

use super::Theme;
use super::container::{Spec, Style};
use super::media::Frame;
use super::tweak::Tweak;
use crate::{Border, radius};

/// Icon media paint: a muted rounded tile with foreground text.
pub fn icon(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground),
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// Icon media variant: the content centered on a 32px muted tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Icon;

impl From<Icon> for Frame {
    fn from(_: Icon) -> Frame {
        Frame::new(Spec::variant(icon), Some(32.0), false)
    }
}

impl<T: Into<Tweak>> Add<T> for Icon {
    type Output = Frame;

    fn add(self, rhs: T) -> Frame {
        Frame::from(self) + rhs
    }
}
