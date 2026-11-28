//! Keyboard shortcut labels. [`group`] and `kbd::group!` arrange several keycaps with a
//! four-pixel gap.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::kbd;
//!
//! kbd::group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")]
//! kbd::group![kbd("Ctrl"), "+", kbd("B")]
//!
//! // Inside tooltip content.
//! kbd("S").style(kbd::Tooltip)
//! ```

use iced_core::Alignment;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::container::Spec;

use crate::compose::{Compose, Inherited, Slot};
use crate::row::Row;
use crate::{Size, Weight};

pub use crate::__kbd_group as group;
pub use icedcn_core::theme::kbd::{Default, Tooltip};

/// Lay out keys in a kbd group.
///
/// ```ignore
/// kbd::group![kbd("Ctrl"), "+", kbd("B")]
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __kbd_group {
    ($($child:expr),+ $(,)?) => {
        $crate::kbd::group([
            $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
        ])
    };
}

/// A keyboard key.
pub struct Kbd<'a, Message> {
    content: Slot<'a, Message>,
    style: Spec,
}

/// Create a [`Kbd`] showing `content`.
pub fn kbd<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Kbd<'a, Message> {
    Kbd {
        content: content.into(),
        style: Default.into(),
    }
}

impl<'a, Message: 'a> Kbd<'a, Message> {
    /// Set the style: `kbd::Tooltip` inside tooltip content, or a tweaked
    /// `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Kbd<'a, Message> {
    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // `text-xs font-medium`, icons at `size-3`.
        self.content.inherit(
            &Inherited::default()
                .with_size(Size::Xs)
                .with_weight(Weight::Medium),
        );

        // `h-5 min-w-5 px-1`, centered.
        crate::container(self.content)
            .height(20)
            .min_width(20)
            .padding([0, 4])
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(self.style)
            .into()
    }
}

impl<'a, Message: 'a> From<Kbd<'a, Message>> for Slot<'a, Message> {
    fn from(kbd: Kbd<'a, Message>) -> Self {
        Slot::new(kbd)
    }
}

impl<'a, Message: 'a> From<Kbd<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(kbd: Kbd<'a, Message>) -> Self {
        Box::new(kbd).compose()
    }
}

/// Lay `keys` out in a row with a 4px gap (`KbdGroup`).
pub fn group<'a, Message: 'a>(
    keys: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Row::with_children(keys)
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
}
