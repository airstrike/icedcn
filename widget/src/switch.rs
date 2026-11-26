//! An on/off switch with an animated thumb and optional label.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::switch;
//!
//! // Uncontrolled: the switch keeps its own value.
//! switch().label("Airplane mode")
//!
//! // Controlled: the app owns the value.
//! switch()
//!     .checked(self.is_on)
//!     .on_checked_change(Message::ToggleAirplane)
//! ```

pub(crate) mod element;
pub mod widget;

pub use icedcn_core::theme::switch::{Default, Disabled, Invalid};
pub use widget::{Catalog, Ring, Status, Style, StyleFn};

use crate::compose::{Compose, Inherited, Slot};
use iced_core::text::IntoFragment;
use iced_core::{Length, Pixels};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme;

use self::element::from_core;

/// Nova switch track dimensions.
#[derive(Default, Debug, Clone, Copy)]
pub enum Size {
    Sm,
    #[default]
    Default,
}

/// A two-state switch.
///
/// Use [`default_checked`](Self::default_checked) to initialize widget-owned state, or
/// [`checked`](Self::checked) with [`on_checked_change`](Self::on_checked_change) to manage it
/// in the application. [`disabled`](Self::disabled) and disabled ancestors prevent interaction.
pub struct Switch<'a, Message> {
    checked: Option<bool>,
    default_checked: bool,
    disabled: bool,
    disabled_opacity: Option<f32>,
    label: Option<iced_core::text::Fragment<'a>>,
    width: Length,
    size: Size,
    track: Option<(f32, f32)>,
    on_checked_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: Option<theme::switch::StyleFn<'a, Theme>>,
}

/// Create an off, uncontrolled [`Switch`].
pub fn switch<'a, Message: 'a>() -> Switch<'a, Message> {
    Switch {
        checked: None,
        default_checked: false,
        disabled: false,
        disabled_opacity: None,
        label: None,
        width: Length::Shrink,
        size: Size::Default,
        track: None,
        on_checked_change: None,
        style: None,
    }
}

impl<'a, Message: 'a> Switch<'a, Message> {
    /// Control the switch: it always shows `checked`, and a click only
    /// publishes [`on_checked_change`](Self::on_checked_change).
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// The initial state of an uncontrolled switch. Later rebuilds keep the
    /// value the user toggled to.
    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    /// Set the message published with the new checked state on every toggle.
    pub fn on_checked_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_checked_change = Some(Box::new(on_change));
        self
    }

    /// Disable the switch.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// On WGPU, paint the active track and thumb together at this opacity while
    /// disabled. The inline label retains its disabled text style. Software rendering
    /// and switches without this option use the style's ordinary disabled appearance.
    pub fn disabled_opacity(mut self, opacity: f32) -> Self {
        self.disabled_opacity = Some(opacity.clamp(0., 1.));
        self
    }

    /// Choose the 24 × 14 or 32 × 18.4 track.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Override the track's width and height, for a style whose track is not
    /// one of Nova's sizes. The thumb follows the style's `padding_ratio`.
    pub fn track(mut self, width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        self.track = Some((width.into().0, height.into().0));
        self
    }

    /// Set the optional inline label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Set the switch's outer width. Defaults to `Length::Shrink`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Override the switch's style. Accepts our OKLCH-native variants
    /// (`Default`, `Invalid`, `Disabled`) or any `Spec`. Ring is preserved
    /// across the crate boundary by the `from_core` bridge in this widget's
    /// `element` submodule.
    pub fn style(mut self, style: impl Into<theme::switch::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl<'a, Message: 'a> From<Switch<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(s: Switch<'a, Message>) -> Self {
        let tg: widget::Toggler<'a, Message, Theme, Renderer> = match s.checked {
            Some(checked) => widget::Toggler::new(checked),
            None => widget::Toggler::new(false).uncontrolled(s.default_checked),
        };
        let mut tg = tg.width(s.width).disabled(s.disabled);
        #[cfg(feature = "wgpu")]
        if let Some(opacity) = s.disabled_opacity {
            tg.disabled_opacity = Some((opacity, element::supports_opacity));
        }
        if let Size::Sm = s.size {
            tg = tg.track_size(24., 14.);
        }
        if let Some((width, height)) = s.track {
            tg = tg.track_size(width, height);
        }

        if let Some(label) = s.label {
            tg = tg.label(label);
        }

        if let Some(style) = s.style {
            // Wrap the core StyleFn so it produces widget::Style (with ring).
            // Status types match across crates (vendored re-exports core's),
            // so no Status conversion is needed here.
            let bridge: widget::StyleFn<'a, Theme> =
                Box::new(move |theme, status| from_core(style(theme, status)));
            tg = tg.class(bridge);
        }

        if let Some(on_change) = s.on_checked_change {
            tg = tg.on_toggle(on_change);
        }

        tg.into()
    }
}

impl<'a, Message: 'a> From<Switch<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Switch<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Switch<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
