//! Style tweaks: small changes applied on top of a variant with `+`.
//!
//! Every tweak is a function returning a [`Tweak`] (`bg(Token::Muted)`,
//! `radius(0)`, `hover(..)`), and the theme roles also come as the
//! [`Bg`] and [`Fg`] enums (`Bg::Muted`). Per-widget `Spec`s consume tweaks
//! when they resolve and drop the ones that do not apply, like CSS.
//! [`crate::theme`] re-exports all of them.
//!
//! `Radius` and `Shadow` are tweaks as they are, through the `From` and
//! `Add` impls at the bottom of this module.

use std::ops::Add;

use crate::{Radius, Shadow};

pub mod bg;
pub mod border;
pub mod fg;
pub mod paint;
pub mod state;
pub mod text;
pub mod token;

pub use bg::{Bg, bg};
pub use fg::{Fg, fg};
pub use paint::{alpha, opaque, ring};
pub use state::{active, disabled, focus, hover};
pub use text::{align, family, italic, line_height, size, weight};
pub use token::Token;

pub use crate::radius;

/// A single style tweak. Per-widget `Spec`s match on the variants they
/// care about and ignore the rest.
#[derive(Debug, Clone)]
pub enum Tweak {
    Bg(Token),
    Fg(Token),
    BorderColor(Token),
    BorderWidth(f32),
    Radius(Radius),
    Shadow(Shadow),
    Ring {
        color: Token,
        width: f32,
        offset: f32,
    },
    Opaque,
    Alpha(f32),
    Size(iced_core::Pixels),
    Weight(iced_core::font::Weight),
    Family(iced_core::Font),
    LineHeight(f32),
    Italic,
    Align(iced_core::alignment::Horizontal),
    State(Trigger, TweakSet),
}

/// An accumulator of tweaks, built up with `+`.
#[derive(Debug, Clone, Default)]
pub struct TweakSet(pub(crate) Vec<Tweak>);

/// State trigger for conditional tweaks (`hover(...)`, `focus(...)`,
/// `active(...)`, `disabled(...)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    Hover,
    Focus,
    Active,
    Disabled,
}

impl<T: Into<Tweak>> From<T> for TweakSet {
    fn from(t: T) -> TweakSet {
        TweakSet(vec![t.into()])
    }
}

impl TweakSet {
    /// The tweaks in the order they apply.
    pub fn iter(&self) -> std::slice::Iter<'_, Tweak> {
        self.0.iter()
    }
}

impl IntoIterator for TweakSet {
    type Item = Tweak;
    type IntoIter = std::vec::IntoIter<Tweak>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a TweakSet {
    type Item = &'a Tweak;
    type IntoIter = std::slice::Iter<'a, Tweak>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<R: Into<Tweak>> Add<R> for Tweak {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self, rhs.into()])
    }
}

impl<R: Into<Tweak>> Add<R> for TweakSet {
    type Output = TweakSet;
    fn add(mut self, rhs: R) -> TweakSet {
        self.0.push(rhs.into());
        self
    }
}

// --- Primitive bridges -------------------------------------------------------
//
// `Radius` and `Shadow` are tweaks as they are: the `From` impls turn them
// into a `Tweak` and the `Add` impls let them start a `+` chain.

impl From<Radius> for Tweak {
    fn from(r: Radius) -> Self {
        Tweak::Radius(r)
    }
}

impl<R: Into<Tweak>> Add<R> for Radius {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}

impl From<Shadow> for Tweak {
    fn from(s: Shadow) -> Self {
        Tweak::Shadow(s)
    }
}

impl<R: Into<Tweak>> Add<R> for Shadow {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
