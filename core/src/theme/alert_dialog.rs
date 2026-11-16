//! AlertDialog styling - nova style.
//!
//! The panel is a dialog's (`rounded-xl bg-popover ring-1`). Its footer is
//! `bg-muted/50` with the panel's bottom corners, below a `border-t`; its
//! media is `size-10 rounded-md bg-muted`.

use std::ops::Add;

use super::Theme;
use super::container::{self, Style};
use super::tweak::Tweak;
use super::variant::spec;
use crate::{Border, Radius, radius};

/// The footer band.
pub fn footer(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.5).into()),
        border: Border {
            radius: Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: radius::XL,
                bottom_left: radius::XL,
            },
            ..Border::default()
        },
        ..Style::default()
    }
}

/// The media tile beside or above the title.
pub fn media(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        text_color: Some(theme.neutral.foreground),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// The media tile's style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Media;

pub use super::variant::Destructive;

/// A media tile's paint: [`Media`] or [`Destructive`], plus tweaks.
#[derive(Debug, Clone)]
pub struct Spec {
    paint: container::Spec,
}

impl Spec {
    /// Resolve this spec to a concrete `Style` for the given theme.
    pub fn resolve(&self, theme: &Theme) -> Style {
        self.paint.resolve(theme)
    }
}

impl From<Media> for Spec {
    fn from(_: Media) -> Spec {
        Spec {
            paint: container::Spec::new(media),
        }
    }
}

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec {
            paint: container::Spec::new(|theme| {
                let mut style = media(theme);
                let opacity = if theme.neutral.is_dark { 0.2 } else { 0.1 };
                style.background = Some(theme.destructive.base.alpha(opacity).into());
                style.text_color = Some(theme.destructive.base);
                style
            }),
        }
    }
}

impl<T: Into<Tweak>> Add<T> for Media {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self) + rhs
    }
}

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec {
            paint: self.paint + rhs,
        }
    }
}

spec!(Spec, Media);

impl From<Spec> for container::Spec {
    fn from(spec: Spec) -> container::Spec {
        spec.paint
    }
}

impl<'a> From<Spec> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        spec.paint.into()
    }
}
