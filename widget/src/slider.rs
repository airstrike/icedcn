//! Slider — vendored iced slider with nova cursor and hover-ring behavior.

use std::ops::RangeInclusive;

pub(crate) mod element;
pub mod widget;

pub use widget::{
    Catalog, Handle, HandleShape, Rail, Ring, Slider, Status, Style, StyleFn, Thumbs,
};

/// Create an uncontrolled, nova-styled slider over `range`. Use
/// [`value`](Slider::value) and [`on_value_change`](Slider::on_value_change)
/// to control it, or [`default_value`](Slider::default_value) to seed it.
pub fn slider<'a, T, Message>(
    range: RangeInclusive<T>,
) -> Slider<'a, T, Message, icedcn_core::Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    Slider::new(range)
}

/// Create two ordered, individually focusable thumbs, initially at the bounds.
pub fn range<'a, T: Copy + PartialOrd, Message: Clone>(
    bounds: RangeInclusive<T>,
) -> Slider<'a, T, Message, icedcn_core::Theme, [T; 2]> {
    let values = [*bounds.start(), *bounds.end()];
    Slider::from_values(bounds, values)
}

/// Create a slider with one thumb for each initial value.
pub fn multiple<'a, T: Copy + PartialOrd, Message: Clone, const N: usize>(
    bounds: RangeInclusive<T>,
    values: [T; N],
) -> Slider<'a, T, Message, icedcn_core::Theme, [T; N]> {
    Slider::from_values(bounds, values).default_value(values)
}
