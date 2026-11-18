//! Button components
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::button;
//!
//! button("Submit").on_press(Message::Submit)
//! button("Go").size(Size::Lg).on_press(Message::Go)
//! button![icon::git_branch(), "New Branch"].on_press(Message::Go)
//! button("Outline").style(button::Outline).on_press(Message::Nav)
//! button!["Outline ", icon::arrow_right()].style(button::Outline)
//! button(icon::arrow_up()).square().rounded().style(button::Outline)
//! ```
//!
//! # Variants
//!
//! Choose a variant with `.style(button::Outline)` or customize a variant with theme tweaks.

mod element;
pub mod group;
pub mod widget;

pub use crate::__button_group as group;
pub use element::{Button, button};
pub use group::{ButtonGroup, group};
pub use icedcn_core::theme::button::{
    Default, Destructive, Ghost, IntoStyle, Link, Outline, Secondary,
};

/// Create a button with one or more children.
///
/// Two forms:
/// - `button!("Submit")` — single child, defers to [`button`].
/// - `button![child1, child2, ...]` — multiple children composed in an inline row.
///   The button supplies the size variant's gap after inheritance.
///
/// The variant is a style, set after the macro:
///
/// ```ignore
/// button![icon::mail(), "Login with Email"]
/// button![icon::git_branch(), "Branch"].style(button::Outline)
/// button!["Accept", kbd("⏎")].style(button::Outline)
/// ```
#[macro_export]
macro_rules! button {
    // Single-arg arm
    ($single:expr $(,)?) => {
        $crate::button::button($single)
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        $crate::button::button($first)$(.push($rest))+
    };
}

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke the macro single-arg and multi-arg,
    //! and verify the result is a `Button`.

    use crate::button::{self, Button};

    #[derive(Clone)]
    enum Msg {}

    #[test]
    fn macro_arms_compile() {
        let _: Button<'_, Msg> = crate::button!("default");
        let _: Button<'_, Msg> = crate::button!["a", "b"].style(button::Outline);
        let _: Button<'_, Msg> = crate::button!("d").style(button::Destructive);
    }

    /// A leading identifier is an ordinary expression: the local itself.
    #[test]
    fn leading_ident_is_an_expression() {
        let label = "Submit";
        let _: Button<'_, Msg> = crate::button![label, "Now"];
    }
}
