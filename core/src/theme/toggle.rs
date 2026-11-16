//! Toggle styling, shared by `Toggle` and `ToggleGroup` items.
//!
//! Active toggles use a muted fill. The outline variant adds a border. Both accept button [`Spec`] tweaks.

use std::ops::Add;

use super::Theme;
use super::button::{self, Status, Style};
use super::tweak::Tweak;
use super::variant::spec;
use crate::{Border, Color, radius};

/// Default toggle: no fill at rest, muted on hover.
pub fn default(theme: &Theme, status: Status) -> Style {
    let base = Style {
        background: None,
        text_color: theme.neutral.foreground,
        border: Border {
            radius: radius::MD.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => Style {
            background: Some(theme.muted.base.into()),
            ..base
        },
        Status::Disabled => Style {
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}

/// Outline toggle: the default toggle inside a `border-input` stroke.
pub fn outline(theme: &Theme, status: Status) -> Style {
    let mut style = default(theme, status);
    let border = if status == Status::Disabled {
        theme.input.scale_alpha(0.5)
    } else {
        theme.input
    };

    style.border.width = 1.0;
    style.border.color = border;
    style
}

/// Paint `style` as pressed on (`data-[state=on]:bg-muted`).
pub fn on(theme: &Theme, style: Style) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        text_color: theme.neutral.foreground,
        ..style
    }
}

/// Default toggle variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

pub use super::variant::Outline;

/// A toggle's style: a variant plus tweaks, resolved as a button [`button::Spec`].
#[derive(Debug, Clone)]
pub struct Spec {
    paint: button::Spec,
}

impl Spec {
    /// Resolve this spec to a concrete `Style` for the given theme and status.
    pub fn resolve(&self, theme: &Theme, status: Status) -> Style {
        self.paint.resolve(theme, status)
    }
}

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec {
            paint: button::Spec::new(default),
        }
    }
}

impl From<Outline> for Spec {
    fn from(_: Outline) -> Spec {
        Spec {
            paint: button::Spec::new(outline),
        }
    }
}

impl From<button::Spec> for Spec {
    fn from(paint: button::Spec) -> Spec {
        Spec { paint }
    }
}

impl From<Spec> for button::Spec {
    fn from(spec: Spec) -> button::Spec {
        spec.paint
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
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

spec!(Spec, Default);
