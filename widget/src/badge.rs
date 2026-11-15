//! Badge component — small inline pill label
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::badge;
//!
//! badge("Badge")
//! badge("Secondary").style(badge::Secondary)
//! badge("Destructive").style(badge::Destructive)
//! badge("Outline").style(badge::Outline)
//! badge("Ghost").style(badge::Ghost)
//! badge("Open Link").style(badge::Secondary).on_press(Message::Open)
//!
//! // Macro form: single-arg defers to [`badge`]; multi-arg wraps children
//! // in a `row::Row::with_children([...]).spacing(4)` to land the nova
//! // `gap-1` (4px) icon-text gap automatically.
//! badge!["Verified", icon::badge_check()]
//! badge!["Verified", icon::badge_check()].style(badge::Secondary)
//! badge!["Bookmark", icon::bookmark()].style(badge::Outline)
//! ```
//!
//! # Variants
//!
//! Pass a variant such as `badge::Outline` to `.style(..)`.

mod widget;

use iced_widget::Widget;
/// Create a badge with one or more children.
///
/// Two forms:
/// - `badge!("Badge")` — single child, defers to [`badge`].
/// - `badge![child1, child2, ...]` — multiple children, wrapped in a
///   row with `spacing(4)` (nova `gap-1`) and passed to [`badge`].
///
/// The variant is a style, set after the macro:
///
/// ```ignore
/// badge!["Verified", icon::badge_check()]
/// badge!["Verified", icon::badge_check()].style(badge::Secondary)
/// badge![spinner(), "Deleting"].style(badge::Destructive)
/// badge![badge_check, "Verified"]                    // leading-ident sugar
/// ```
#[macro_export]
macro_rules! badge {
    // Single-arg arm
    ($single:expr $(,)?) => {
        $crate::badge::badge($single)
    };
    // Leading-ident icon sugar: the first arg as a bare ident is rewritten
    // to `icon::$first()`. Requires `use crate::icon;` (or equivalent path
    // to an `icon` module exposing `fn $first() -> Text`) in scope at the
    // call site. Macro hygiene: `icon` here resolves at the call site, not
    // the def site, as in `button!`.
    //
    // `badge![badge_check, "Verified"]` => badge(row![icon::badge_check(), "Verified"].spacing(4))
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    Into::<$crate::compose::Slot<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::compose::Slot<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        ).icon_start()
    };
    // Multi-arg arm — wraps children in a row with 4px gap.
    ($($child:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

use crate::Weight;
use crate::compose::{Compose, Inherited, Slot};
use iced_core::Padding;
use iced_widget::Renderer;
use icedcn_core::Theme;

pub use icedcn_core::theme::badge::state::{IntoStyle, Status, Style, StyleFn};
pub use icedcn_core::theme::badge::{Default, Destructive, Ghost, Link, Outline, Secondary};

/// A small inline pill label.
pub struct Badge<'a, Message> {
    content: Slot<'a, Message>,
    style: StyleFn<'a>,
    on_press: Option<Message>,
    start_icon: bool,
    end_icon: bool,
    disabled: bool,
    invalid: bool,
    inherited: Inherited,
}

/// Create a [`Badge`] with the default variant.
pub fn badge<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Badge<'a, Message> {
    Badge {
        content: content.into(),
        style: Default.into_style(),
        on_press: None,
        start_icon: false,
        end_icon: false,
        disabled: false,
        invalid: false,
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Badge<'a, Message> {
    /// Tighten the inset on the side of an inline icon.
    pub fn icon_start(mut self) -> Self {
        self.start_icon = true;
        self
    }
    pub fn icon_end(mut self) -> Self {
        self.end_icon = true;
        self
    }
    /// Select a Nova variant, tweaked Spec, or legacy static container callback.
    ///
    /// Static callbacks supply exact paint in every state, without inferred
    /// hover/focus effects. The last `style` or `state_style` call wins.
    pub fn style(mut self, style: impl IntoStyle<'a>) -> Self {
        self.style = style.into_style();
        self
    }

    /// Supply all state-dependent paint, replacing the selected variant/style.
    pub fn state_style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Paint the Nova invalid border and, when keyboard-focused, its ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Suppress interaction, including hover and keyboard focus.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled |= disabled;
        self
    }
}

impl<'a, Message: Clone + 'a> Badge<'a, Message> {
    /// Show a pointer cursor and emit `message` when the user releases the badge inside its
    /// bounds.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Badge<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(badge: Badge<'a, Message>) -> Self {
        Box::new(badge).compose()
    }
}

impl<'a, Message: Clone + 'a> From<Badge<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Badge<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Badge<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // Resolve our typography before forwarding the cascade: Text keeps the
        // first inherited value, so ancestor line heights must not arrive first.
        // Nova fixes the outer height at 20px with a 16px text line box.
        let mut inherited = self
            .inherited
            .with_size(crate::Size::Xs)
            .with_weight(Weight::Medium)
            .with_line_height(16.0)
            .with_disabled(self.disabled);
        if self.on_press.is_some() {
            inherited = inherited.with_selectable(false);
        }
        self.content.inherit(&inherited);

        let pad = Padding {
            top: 2.0,
            bottom: 2.0,
            // Iced paints borders inside the layout box. Reserve the CSS
            // border pixel in addition to the horizontal padding.
            left: if self.start_icon { 7.0 } else { 9.0 },
            right: if self.end_icon { 7.0 } else { 9.0 },
        };
        widget::Badge {
            content: self.content.compose(),
            style: self.style,
            on_press: self.on_press,
            disabled: self.disabled,
            invalid: self.invalid,
            padding: pad,
        }
        .boxed()
    }
}

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke the macro single-arg, multi-arg and
    //! with the leading-ident sugar, and verify the result is a [`Badge`].

    use crate::badge::{self, Badge};

    #[derive(Clone)]
    enum Msg {}

    #[test]
    fn macro_arms_compile() {
        let _: Badge<'_, Msg> = crate::badge!("default");
        let _: Badge<'_, Msg> = crate::badge!["a", "b"].style(badge::Secondary);
        let _: Badge<'_, Msg> = crate::badge!("o").style(badge::Outline);
    }

    /// Leading-ident sugar: `badge![my_icon, "Tag"]` rewrites the bare
    /// ident to `icon::my_icon()` at the call site, then prepends it to the
    /// row of remaining children.
    #[test]
    fn leading_ident_sugar_arm() {
        // Local `icon` module — name load-bearing because the sugar arm
        // expands to `icon::$first()` resolved at the call site.
        mod icon {
            pub fn my_icon<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }

        let _: Badge<'_, Msg> = crate::badge![my_icon, "Tag"];
        let _: Badge<'_, Msg> = crate::badge![my_icon, "Deleting"].style(badge::Destructive);
    }
}
