//! Style variants shared by every component.
//!
//! [`Destructive`] is one type: `.style(Destructive)` picks the destructive
//! look of whichever component receives it, because each component's `Spec`
//! implements `From<Destructive>`. A component without that variant simply
//! has no such impl, so the mismatch is a compile error.
//!
//! `Variant + tweak` builds a [`Tweaked`], which any `Spec` that converts
//! from the variant also converts from:
//!
//! ```
//! use icedcn_core::theme::tweak::{Bg, Token};
//! use icedcn_core::theme::{Destructive, badge, button};
//!
//! let on_button: button::Spec = (Destructive + Bg(Token::Muted)).into();
//! let on_badge: badge::Spec = (Destructive + Bg(Token::Muted)).into();
//! # let _ = (on_button, on_badge);
//! ```
//!
//! # Custom themes
//!
//! Variants paint from the theme's palettes, so a [`Design`](crate::Design)
//! with its own `destructive` palette restyles [`Destructive`] on every
//! component. To change a variant's shape for a whole app, name the tweaked
//! variant once; it still works on every component:
//!
//! ```
//! use icedcn_core::theme::tweak::radius;
//! use icedcn_core::theme::{Destructive, Tweaked, alert, button};
//! use icedcn_core::{Design, Theme, oklch};
//!
//! let mut design = Design::dark();
//! design.destructive.base = oklch(0.6, 0.2, 280.0);
//! let theme = Theme::new(design);
//!
//! let alert = alert::Spec::from(Destructive).resolve(&theme);
//! let button = button::Spec::from(Destructive).resolve(&theme, button::Status::Active);
//! assert_eq!(alert.text_color, Some(theme.destructive.base));
//! assert_eq!(button.text_color, theme.destructive.base);
//!
//! fn danger() -> Tweaked<Destructive> {
//!     Destructive + radius(0)
//! }
//! let square: button::Spec = danger().into();
//! let square = square.resolve(&theme, button::Status::Active);
//! assert_eq!(square.border.radius, radius(0));
//! ```
//!
//! # Custom components and variants
//!
//! A component of your own accepts the shared variants by converting its
//! `Spec` from them, and [`Tweaked::build`] applies the tweaks. A variant of
//! your own comes from [`variant!`](crate::variant), which gives it the same
//! `+`:
//!
//! ```
//! use std::ops::Add;
//!
//! use icedcn_core::theme::tweak::{Bg, Token, Tweak};
//! use icedcn_core::theme::{Destructive, Tweaked, button};
//! use icedcn_core::{Color, Theme};
//!
//! icedcn_core::variant! {
//!     /// The brand's call to action.
//!     pub struct Brand;
//! }
//!
//! /// A meter's paint: its fill and the tweaks applied on top.
//! #[derive(Clone)]
//! struct Meter {
//!     fill: fn(&Theme) -> Color,
//!     tweaks: Vec<Tweak>,
//! }
//!
//! impl Meter {
//!     fn resolve(&self, theme: &Theme) -> Color {
//!         self.tweaks.iter().fold((self.fill)(theme), |color, tweak| match tweak {
//!             Tweak::Bg(token) => token.resolve(theme),
//!             _ => color,
//!         })
//!     }
//! }
//!
//! impl<T: Into<Tweak>> Add<T> for Meter {
//!     type Output = Meter;
//!     fn add(mut self, tweak: T) -> Meter {
//!         self.tweaks.push(tweak.into());
//!         self
//!     }
//! }
//!
//! impl From<Destructive> for Meter {
//!     fn from(_: Destructive) -> Meter {
//!         Meter { fill: |theme| theme.destructive.base, tweaks: Vec::new() }
//!     }
//! }
//!
//! impl From<Brand> for Meter {
//!     fn from(_: Brand) -> Meter {
//!         Meter { fill: |theme| theme.chart[0], tweaks: Vec::new() }
//!     }
//! }
//!
//! impl<V> From<Tweaked<V>> for Meter
//! where
//!     Meter: From<V>,
//! {
//!     fn from(tweaked: Tweaked<V>) -> Meter {
//!         tweaked.build()
//!     }
//! }
//!
//! // Brand on an icedcn component: a Spec built from your own paint.
//! impl From<Brand> for button::Spec {
//!     fn from(_: Brand) -> button::Spec {
//!         button::Spec::new(button::secondary)
//!     }
//! }
//!
//! let theme = Theme::dark();
//! let meter = Meter::from(Destructive + Bg(Token::Primary));
//! assert_eq!(meter.resolve(&theme), theme.primary.base);
//! assert_eq!(Meter::from(Brand).resolve(&theme), theme.chart[0]);
//!
//! let tweaked: button::Spec = (Brand + Bg(Token::Muted)).into();
//! let style = tweaked.resolve(&theme, button::Status::Active);
//! assert_eq!(style.background, Some(theme.muted.base.into()));
//! ```

use std::ops::Add;

use super::tweak::{Tweak, TweakSet};

/// A variant with tweaks applied on top, built by `Variant + tweak`.
///
/// Any `Spec` that converts from `V` converts from `Tweaked<V>` too.
#[derive(Debug, Clone)]
pub struct Tweaked<V> {
    /// The base variant.
    pub variant: V,
    /// The tweaks, applied in order after the variant.
    pub tweaks: TweakSet,
}

impl<V> Tweaked<V> {
    /// Apply `tweaks` on top of `variant`.
    pub fn new(variant: V, tweaks: impl Into<TweakSet>) -> Self {
        Self {
            variant,
            tweaks: tweaks.into(),
        }
    }

    /// Build `S` from the variant, then add each tweak in order.
    pub fn build<S>(self) -> S
    where
        S: From<V> + Add<Tweak, Output = S>,
    {
        self.tweaks
            .into_iter()
            .fold(S::from(self.variant), |spec, tweak| spec + tweak)
    }
}

impl<V, T: Into<Tweak>> Add<T> for Tweaked<V> {
    type Output = Tweaked<V>;
    fn add(mut self, rhs: T) -> Tweaked<V> {
        self.tweaks = self.tweaks + rhs;
        self
    }
}

/// Define a unit-struct style variant whose `+` builds a [`Tweaked`].
///
/// ```
/// use icedcn_core::theme::tweak::Opaque;
///
/// icedcn_core::variant! {
///     /// A highlighted surface.
///     pub struct Highlight;
/// }
///
/// let _ = Highlight + Opaque;
/// ```
#[macro_export]
macro_rules! variant {
    ($(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        $vis struct $name;

        impl<T: Into<$crate::theme::tweak::Tweak>> ::core::ops::Add<T> for $name {
            type Output = $crate::theme::Tweaked<$name>;
            fn add(self, rhs: T) -> $crate::theme::Tweaked<$name> {
                $crate::theme::Tweaked::new(self, rhs)
            }
        }
    };
}

/// Implement the shared conversions into a component's `Spec`: from
/// [`Tweaked`] for each variant the `Spec` accepts and, given its default
/// variant, from bare tweaks and tweak sets applied on that default.
macro_rules! spec {
    ($spec:ty) => {
        impl<V> From<$crate::theme::Tweaked<V>> for $spec
        where
            $spec: From<V>,
        {
            fn from(tweaked: $crate::theme::Tweaked<V>) -> Self {
                tweaked.build()
            }
        }
    };
    ($spec:ty, $default:expr) => {
        spec!($spec);

        impl From<$crate::theme::tweak::TweakSet> for $spec {
            fn from(tweaks: $crate::theme::tweak::TweakSet) -> Self {
                $crate::theme::Tweaked::new($default, tweaks).build()
            }
        }

        impl<T: Into<$crate::theme::tweak::Tweak>> From<T> for $spec {
            fn from(tweak: T) -> Self {
                $crate::theme::Tweaked::new($default, tweak).build()
            }
        }
    };
}

/// Implement conversions into a style closure type (`$sink`) through the
/// component's `Spec`, for widgets whose `.style(..)` takes the closure.
/// `tweaks` also converts bare tweak sets, for a `Spec` with a default.
macro_rules! sink {
    ($sink:ty, $spec:ty) => {
        impl<'a, V> From<$crate::theme::Tweaked<V>> for $sink
        where
            $spec: From<V>,
        {
            fn from(tweaked: $crate::theme::Tweaked<V>) -> Self {
                tweaked.build::<$spec>().into()
            }
        }
    };
    ($sink:ty, $spec:ty, tweaks) => {
        sink!($sink, $spec);

        impl<'a> From<$crate::theme::tweak::TweakSet> for $sink {
            fn from(tweaks: $crate::theme::tweak::TweakSet) -> Self {
                <$spec>::from(tweaks).into()
            }
        }
    };
}

pub(crate) use {sink, spec};

variant! {
    /// The destructive look: errors and dangerous actions (`variant="destructive"`).
    pub struct Destructive;
}

variant! {
    /// A bordered look on the page background (`variant="outline"`).
    pub struct Outline;
}

variant! {
    /// No fill at rest (`variant="ghost"`).
    pub struct Ghost;
}

variant! {
    /// The secondary surface (`variant="secondary"`).
    pub struct Secondary;
}

variant! {
    /// Styled as a link in the primary color (`variant="link"`).
    pub struct Link;
}

variant! {
    /// The muted surface or text color (`variant="muted"`).
    pub struct Muted;
}

variant! {
    /// A form control with an invalid value (`aria-invalid`).
    pub struct Invalid;
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::super::tweak::{Alpha, Bg, Fg, Token};
    use super::super::{Theme, alert, alert_dialog, badge, button, container, text, toggle};
    use super::*;
    use crate::Background;

    #[test]
    fn component_paths_name_the_shared_types() {
        let destructive = TypeId::of::<Destructive>();
        assert_eq!(TypeId::of::<button::Destructive>(), destructive);
        assert_eq!(TypeId::of::<badge::Destructive>(), destructive);
        assert_eq!(TypeId::of::<alert::Destructive>(), destructive);
        assert_eq!(TypeId::of::<text::Destructive>(), destructive);
        assert_eq!(
            TypeId::of::<button::Outline>(),
            TypeId::of::<toggle::Outline>()
        );
        assert_eq!(TypeId::of::<badge::Link>(), TypeId::of::<text::Link>());
    }

    #[test]
    fn one_tweaked_variant_resolves_per_component() {
        let theme = Theme::dark();
        let muted: Background = theme.muted.base.into();
        let tweaked = || Destructive + Bg(Token::Muted);

        let on_button: button::Spec = tweaked().into();
        let on_button = on_button.resolve(&theme, button::Status::Active);
        assert_eq!(on_button.background, Some(muted));
        assert_eq!(on_button.text_color, theme.destructive.base);

        let on_badge: badge::Spec = tweaked().into();
        assert_eq!(on_badge.resolve(&theme).background, Some(muted));

        let on_alert: alert::Spec = tweaked().into();
        let on_alert = on_alert.resolve(&theme);
        assert_eq!(on_alert.background, Some(muted));
        assert_eq!(on_alert.text_color, Some(theme.destructive.base));

        let on_media: alert_dialog::Spec = tweaked().into();
        assert_eq!(on_media.resolve(&theme).background, Some(muted));

        let on_panel: container::StyleFn<'_, Theme> = tweaked().into();
        assert_eq!(on_panel(&theme).background, Some(muted));

        let faded: text::Spec = (Destructive + Alpha(0.5)).into();
        assert_eq!(
            faded.resolve(&theme).color,
            Some(theme.destructive.base.scale_alpha(0.5))
        );
    }

    #[test]
    fn bare_tweaks_apply_on_the_default_variant() {
        let theme = Theme::light();
        let status = button::Status::Active;
        let bare = button::Spec::from(Bg(Token::Muted)).resolve(&theme, status);
        let explicit = (button::Default + Bg(Token::Muted)).resolve(&theme, status);
        assert_eq!(bare, explicit);

        let set = button::Spec::from(Bg(Token::Muted) + Fg(Token::Primary)).resolve(&theme, status);
        assert_eq!(set.text_color, theme.primary.base);
        assert_eq!(set.background, Some(theme.muted.base.into()));
    }
}
