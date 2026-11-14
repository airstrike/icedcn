//! Avatar — circular image with text fallback.
//!
//! Mirrors shadcn nova's `<Avatar>`/`<AvatarImage>`/`<AvatarFallback>` triple.
//! Three sizes (`Sm` 24px, `Default` 32px, `Lg` 40px), an optional corner
//! [`badge`], and a [`group`] constructor for negative-space overlap with an
//! optional trailing count chip via [`group::Group::count`].
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::avatar;
//!
//! // Stand-alone, with an image and an initials fallback.
//! avatar(Some(my_image_handle)).fallback("CN")
//!
//! // No image — the fallback initials always render.
//! avatar(None).fallback("CN")
//!
//! // Sized — `.size(avatar::Lg)` for 40px, `.size(avatar::Sm)` for 24px.
//! avatar(None).fallback("ER").size(avatar::Lg)
//!
//! // Corner badge (presence dot).
//! avatar::badge(avatar(None).fallback("CN"), avatar::dot())
//!
//! // Group with three avatars and a `+3` count.
//! avatar::group([
//!     avatar(None).fallback("CN"),
//!     avatar(None).fallback("LR"),
//!     avatar(None).fallback("ER"),
//! ])
//! .count("+3")
//! ```
//!
//! The fallback is visible while the renderer decodes the image and if it fails.

pub mod badge;
pub mod group;
mod widget;

use iced_core::Length::Fill;
use iced_core::image::Handle;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::avatar::badge_style;
use icedxdy_core::theme::container::{Style, StyleFn};

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;

#[doc(inline)]
pub use badge::{badge, dot};
#[doc(inline)]
pub use group::group;

// Re-export the size variant unit structs so callers reach them via
// `avatar::Sm`, `avatar::Default`, `avatar::Lg` — the same name shape the
// theme catalog uses, kept consistent across the call site.
#[doc(inline)]
pub use icedxdy_core::theme::avatar::{Default, Geometry, Lg, Sm};

/// Size variant for an [`Avatar`].
///
/// Stored as the same `Geometry` tuple the theme exposes so we don't
/// duplicate diameter / font-size / badge-size constants on the widget
/// side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size(Geometry);

impl Size {
    /// Resolved geometry for this size — diameter, font size, and badge
    /// chip dimensions in logical pixels.
    pub fn geometry(self) -> Geometry {
        self.0
    }
}

impl From<f32> for Size {
    fn from(diameter: f32) -> Self {
        Self(Geometry {
            diameter: diameter.max(0.0),
            ..Geometry::DEFAULT
        })
    }
}

impl From<Default> for Size {
    fn from(_: Default) -> Self {
        Self(Geometry::DEFAULT)
    }
}

impl From<Sm> for Size {
    fn from(_: Sm) -> Self {
        Self(Geometry::SM)
    }
}

impl From<Lg> for Size {
    fn from(_: Lg) -> Self {
        Self(Geometry::LG)
    }
}

impl std::default::Default for Size {
    fn default() -> Self {
        Self(Geometry::DEFAULT)
    }
}

/// Builder for an avatar widget.
///
/// Born from an `Option<image::Handle>`: `Some(handle)` renders the image
/// (clipped to a circle); `None` renders [`fallback`](Self::fallback) text
/// centered on the muted surface.
pub struct Avatar<'a, Message> {
    image: Option<Handle>,
    fallback: Option<Slot<'a, Message>>,
    size: Size,
    /// 2px outer ring colored to the page background. Set by
    /// [`group`](crate::avatar::group) on every avatar it composes so
    /// overlapping avatars look visually separated. Stand-alone avatars
    /// leave this `false`.
    ring: bool,
    badge: Option<Slot<'a, Message>>,
    style: Option<StyleFn<'a, Theme>>,
    badge_style: StyleFn<'a, Theme>,
    inherited: Inherited,
}

/// Create an [`Avatar`] from an optional image handle.
///
/// Pass `None` to always render the fallback content; pass `Some(handle)`
/// to render the image (clipped to a circle).
pub fn avatar<'a, Message: 'a>(image: Option<Handle>) -> Avatar<'a, Message> {
    Avatar {
        image,
        fallback: None,
        size: Size::default(),
        ring: false,
        badge: None,
        style: None,
        badge_style: Box::new(|theme| {
            let mut style = badge_style(theme);
            style.border.width = 0.0;
            style
        }),
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Avatar<'a, Message> {
    /// Set the content shown while an image loads, if decoding fails, or without an image.
    ///
    /// Typically the user's initials (`"CN"`, `"ER"`) but accepts any
    /// `Slot` — an icon, a single character, etc.
    pub fn fallback(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.fallback = Some(content.into());
        self
    }

    /// Set the avatar's size variant. Defaults to [`Default`] (32px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Customize the circular surface, fallback color and inset border.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.style = Some(Box::new(style));
        self
    }

    /// Customize the badge surface and text color, keeping its outer ring.
    pub fn badge_style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.badge_style = Box::new(style);
        self
    }

    /// Resolved geometry for this avatar — exposed so [`badge`] and
    /// [`group`] can compose the corner badge / count chip at the right
    /// dimensions without re-parsing the size.
    pub fn geometry(&self) -> Geometry {
        self.size.geometry()
    }

    /// Toggle the 2px outer ring. Internal API: [`group`] flips this on
    /// for every avatar it composes; stand-alone avatars don't expose it
    /// to keep the builder surface focused on shadcn parity.
    pub(crate) fn ring(mut self, ring: bool) -> Self {
        self.ring = ring;
        self
    }
}

impl<'a, Message: 'a> From<Avatar<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(avatar: Avatar<'a, Message>) -> Self {
        Box::new(avatar).compose()
    }
}

impl<'a, Message: 'a> From<Avatar<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Avatar<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Avatar<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Avatar {
            image,
            fallback,
            size,
            ring,
            badge,
            style,
            badge_style,
            inherited,
        } = *self;
        let geometry = size.geometry();
        let mut fallback = fallback.unwrap_or_else(|| Slot::from(""));
        fallback.inherit(
            &Inherited::default()
                .with_size(crate::Size::Custom(geometry.font_size))
                .with_line_height(if geometry.font_size == 12.0 {
                    16.0
                } else {
                    20.0
                })
                .with_selectable(false)
                .or(inherited),
        );
        let mut children = vec![Container::new(fallback).center(Fill).into()];
        if let Some(mut badge) = badge {
            if geometry.badge_icon_size == 0.0 {
                badge = Slot::from("");
            }
            badge.inherit(
                &Inherited::default()
                    .with_size(crate::Size::Custom(geometry.badge_icon_size))
                    .with_line_height(geometry.badge_icon_size)
                    .with_selectable(false)
                    .or(inherited),
            );
            children.push(Container::new(badge).center(Fill).into());
        }
        widget::Avatar {
            handle: image,
            children,
            geometry,
            ring,
            style,
            badge_style,
        }
        .boxed()
    }
}
