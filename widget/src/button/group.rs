//! A group of buttons joined along a horizontal or vertical axis.
//!
//! # Children are typed
//!
//! Children arrive as [`Item`] values, NOT as `iced::Element`. The group
//! needs to mutate each button's resolved style (corner-radius collapse) AT
//! iteration time — once a child is converted to an `iced::Element`, its
//! style is locked behind the catalog boundary and cannot be touched.
//!
//! Three input shapes flow into the items vec:
//!
//! - Any [`Button`] — converts to [`Item::Button`] via [`From`].
//! - [`separator()`] — yields an [`Item::Separator`] that the group renders
//!   as a 1px divider perpendicular to its orientation.
//! - The [`button_group!`](crate::button_group) macro accepts mixed children and calls `.into()`
//!   on each (matching `row!` / `column!` ergonomics).
//!
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::{button, button_group};
//!
//! button_group![
//!     button("Copy").style(button::Outline),
//!     button::group::separator(),
//!     button("Paste").style(button::Outline),
//! ]
//! ```
//!
//! # Edge-radius collapse
//!
//! The group counts every item, including separators, when it determines which button corners
//! touch neighboring items. It applies the corner adjustment after resolving each button’s style.
//!
//! # Orientation
//!
//! Defaults to [`Horizontal`]. Override with [`ButtonGroup::orientation`].
//! The orientation is also passed down to every [`Item::Separator`] so the
//! divider draws perpendicular to the group axis without the caller spelling
//! it.

use iced_core::Length;
use iced_widget::Renderer;
use iced_widget::Widget;
use icedcn_core::Theme;

use crate::compose::Compose;
use crate::{Button, Slot};

pub mod separator;

pub use separator::Separator;

// Re-export the crate-root [`icedcn::Orientation`] (and its bare variants)
// at this module path. The underlying type is shared across widgets — a
// `field::Orientation` value and a `button::group::Orientation` value are
// the SAME enum, so callers can build `use icedcn::{Vertical, Horizontal};`
// once and feed into every `.orientation()` builder via `impl Into`.
pub use icedcn_core::Horizontal;
pub use icedcn_core::Orientation;
pub use icedcn_core::Vertical;

/// Position of a button among all the items in its group, including separators. This
/// determines which outer corners keep their radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    /// The only child in the group — keeps every corner.
    Only,
    /// First child — keeps "leading" corners (top-left + bottom-left for
    /// horizontal; top-left + top-right for vertical), collapses the
    /// trailing pair.
    First,
    /// Internal child — collapses every corner.
    Middle,
    /// Last child — keeps "trailing" corners, collapses the leading pair.
    Last,
}

/// Decorate a joined button with a tooltip or an overlay after its corners are resolved.
pub fn decorated<'a, Message: 'a>(
    button: Button<'a, Message>,
    decorate: impl FnOnce(Button<'a, Message>) -> Slot<'a, Message> + 'a,
) -> Item<'a, Message> {
    Item::Decorated(button, Box::new(decorate))
}

type Decoration<'a, Message> = Box<dyn FnOnce(Button<'a, Message>) -> Slot<'a, Message> + 'a>;

/// One slot in a [`ButtonGroup`]'s child sequence.
///
/// Constructed via the [`button_group!`](crate::button_group) macro (which calls `.into()` on
/// each child) or directly via [`From`] from a [`Button`] / [`Separator`].
/// Domain-enum shape per the minimap M2 critique: re-styling children means
/// calling [`Button::map_style`] BEFORE the conversion to `iced::Element`,
/// which only the typed enum permits.
pub enum Item<'a, Message> {
    /// A button that participates in the cluster — its corners get
    /// collapsed based on neighbors at iteration time.
    Button(Button<'a, Message>),
    Decorated(Button<'a, Message>, Decoration<'a, Message>),
    Content(Slot<'a, Message>),
    Input(crate::Input<'a, Message>),
    /// A vertical-or-horizontal divider drawn between buttons. The
    /// separator's own axis is set to be perpendicular to the group's
    /// orientation when `ButtonGroup::compose` runs.
    Separator(Separator),
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Item<'a, Message> {
    fn from(b: Button<'a, Message>) -> Self {
        Item::Button(b)
    }
}

impl<'a, Message> From<crate::Input<'a, Message>> for Item<'a, Message> {
    fn from(input: crate::Input<'a, Message>) -> Self {
        Self::Input(input)
    }
}

impl<'a, Message> From<Separator> for Item<'a, Message> {
    fn from(s: Separator) -> Self {
        Item::Separator(s)
    }
}

/// Construct a [`Separator`] item. The group's orientation is patched into
/// the separator at iteration time, so the caller doesn't pass an axis.
pub fn separator() -> Separator {
    Separator::new()
}

/// A horizontally- or vertically-fused cluster of [`Button`]s with optional
/// [`Separator`] dividers.
///
/// Built via [`group()`] or the [`button_group!`](crate::button_group) macro. Configures via
/// [`ButtonGroup::orientation`] and [`ButtonGroup::width`].
pub struct ButtonGroup<'a, Message> {
    items: Vec<Item<'a, Message>>,
    orientation: Orientation,
    width: Option<Length>,
}

/// Build a [`ButtonGroup`] from an ordered sequence of items.
///
/// Prefer the [`button_group!`](crate::button_group) macro — it calls `.into()` on each child
/// so callers don't spell `.into()` per item. This function exists for the
/// macro to delegate to, and for callers building the items vec dynamically.
///
///
/// ```ignore
/// use icedcn::widget::{button, button_group};
///
/// button_group![
///     button("Archive").style(button::Outline),
///     button("Report").style(button::Outline),
///     button("Snooze").style(button::Outline),
/// ]
/// ```
pub fn group<'a, Message: Clone + 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> ButtonGroup<'a, Message> {
    ButtonGroup {
        items: items.into_iter().collect(),
        orientation: Orientation::Horizontal,
        width: None,
    }
}

impl<'a, Message: Clone + 'a> ButtonGroup<'a, Message> {
    /// Set the layout axis. Defaults to [`Horizontal`].
    ///
    /// Horizontal: buttons sit in a row; corners collapse on the inside
    /// (right edge of the first button, both sides of middle buttons, left
    /// edge of the last button).
    ///
    /// Vertical: buttons stack in a column; corners collapse on the inside
    /// (bottom of the first button, top + bottom of middle buttons, top of
    /// the last button).
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Set the outer width of the rendered group. Defaults to shrink-to-content.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<ButtonGroup<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(g: ButtonGroup<'a, Message>) -> Self {
        Box::new(g).compose()
    }
}

impl<'a, Message: Clone + 'a> From<ButtonGroup<'a, Message>> for crate::Slot<'a, Message> {
    fn from(widget: ButtonGroup<'a, Message>) -> Self {
        crate::Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for ButtonGroup<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let ButtonGroup {
            items,
            orientation,
            width,
        } = *self;

        let total = items.len();

        // Bound the separator length by the tallest button so a roomy parent cannot stretch it.
        let cross_axis_size = items
            .iter()
            .filter_map(|item| match item {
                Item::Button(b) | Item::Decorated(b, _) => Some(b.size_height()),
                Item::Separator(_) | Item::Content(_) | Item::Input(_) => None,
            })
            .fold(0.0_f32, f32::max);

        // Convert each item to an iced::Element. Buttons get their style
        // wrapped via `map_style` so the resolved Style has the appropriate
        // corners zeroed; separators get their group_orientation patched.
        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> = items
            .into_iter()
            .enumerate()
            .map(
                |(i, item)| -> iced_core::Element<'a, Message, Theme, Renderer> {
                    let pos = position_at(i, total);
                    let joined = move |b: Button<'a, Message>| {
                        b.map_style(move |s| collapse_corners(s, pos, orientation))
                    };
                    match item {
                        Item::Button(b) => joined(b).into(),
                        Item::Decorated(b, decorate) => decorate(joined(b)).compose(),
                        Item::Content(slot) => slot.compose(),
                        Item::Input(input) => input
                            .group_corners(move |radius| joined_radius(radius, pos, orientation))
                            .into(),
                        Item::Separator(mut sep) => {
                            sep.group_orientation = orientation;
                            sep.cross_axis = cross_axis_size;
                            sep.boxed()
                        }
                    }
                },
            )
            .collect();

        // Adjacent items share an edge along the group axis.
        let element: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Horizontal => iced_widget::Row::with_children(elements)
                .spacing(0)
                .align_y(iced_core::Alignment::Center)
                .boxed(),
            Orientation::Vertical => iced_widget::Column::with_children(elements)
                .spacing(0)
                .align_x(iced_core::Alignment::Start)
                .boxed(),
        };

        match width {
            Some(w) => iced_widget::container(element).width(w).boxed(),
            None => element,
        }
    }
}

/// Compute the [`Position`] of a child at flat index `i` given a total
/// child count `total`. Edge slots collapse to [`Position::Only`] when
/// `total == 1`.
fn position_at(i: usize, total: usize) -> Position {
    if total <= 1 {
        Position::Only
    } else if i == 0 {
        Position::First
    } else if i + 1 == total {
        Position::Last
    } else {
        Position::Middle
    }
}

/// Remove corner radii on edges that face a neighboring group item.
fn collapse_corners(
    mut style: iced_widget::button::Style,
    pos: Position,
    orientation: Orientation,
) -> iced_widget::button::Style {
    style.border.radius = joined_radius(style.border.radius, pos, orientation);
    style
}

fn joined_radius(
    mut radius: iced_core::border::Radius,
    pos: Position,
    orientation: Orientation,
) -> iced_core::border::Radius {
    if pos == Position::Only {
        return radius;
    }
    if matches!(pos, Position::Middle | Position::Last) {
        match orientation {
            Orientation::Horizontal => {
                radius.top_left = 0.0;
                radius.bottom_left = 0.0;
            }
            Orientation::Vertical => {
                radius.top_left = 0.0;
                radius.top_right = 0.0;
            }
        }
    }
    if matches!(pos, Position::Middle | Position::First) {
        match orientation {
            Orientation::Horizontal => {
                radius.top_right = 0.0;
                radius.bottom_right = 0.0;
            }
            Orientation::Vertical => {
                radius.bottom_left = 0.0;
                radius.bottom_right = 0.0;
            }
        }
    }
    radius
}

/// Default-variant button-group macro. Children are passed as a comma-list
/// and each child is `Into::<Item<'_, _>>::into`'d before being collected
/// into the items vec. Mirrors `row!` / `column!` ergonomics so callers
/// don't spell `.into()` per child.
///
/// ```ignore
/// use icedcn::widget::{button, button_group};
///
/// button_group![
///     button("Archive").style(button::Outline),
///     button("Report").style(button::Outline),
///     button("Snooze").style(button::Outline),
/// ]
///
/// // With a separator in the middle — the separator value is `Into<Item>`.
/// button_group![
///     button("Copy").style(button::Secondary),
///     button::group::separator(),
///     button("Paste").style(button::Secondary),
/// ]
/// ```
///
/// `#[macro_export]` is crate-root-scoped (Rust limitation on macros), so
/// the macro is `button_group!` rather than `button::group!`.
#[macro_export]
macro_rules! button_group {
    () => (
        $crate::button::group::group(::core::iter::empty())
    );
    ($($child:expr),+ $(,)?) => (
        $crate::button::group::group([
            $(Into::<$crate::button::group::Item<'_, _>>::into($child)),+
        ])
    );
}

#[cfg(test)]
mod tests {
    //! Position + corner-collapse coverage. The group's `compose`
    //! pipeline can't easily be inspected after the fact (style closures
    //! are erased into iced's catalog), so the reachable surface here is
    //! the pure helpers that compute child position and zero corner radii.

    use super::*;
    use iced_core::Background;
    use iced_widget::button::Style;

    fn dummy_style() -> Style {
        Style {
            background: Some(Background::Color(iced_core::Color::WHITE)),
            text_color: iced_core::Color::BLACK,
            border: iced_core::Border {
                color: iced_core::Color::BLACK,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced_core::Shadow::default(),
            snap: false,
        }
    }

    #[test]
    fn position_at_solo_is_only() {
        assert_eq!(position_at(0, 1), Position::Only);
    }

    #[test]
    fn position_at_pair_is_first_then_last() {
        assert_eq!(position_at(0, 2), Position::First);
        assert_eq!(position_at(1, 2), Position::Last);
    }

    #[test]
    fn position_at_triple_has_middle() {
        assert_eq!(position_at(0, 3), Position::First);
        assert_eq!(position_at(1, 3), Position::Middle);
        assert_eq!(position_at(2, 3), Position::Last);
    }

    #[test]
    fn horizontal_first_keeps_left_zeroes_right() {
        let s = collapse_corners(dummy_style(), Position::First, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn horizontal_last_zeroes_left_keeps_right() {
        let s = collapse_corners(dummy_style(), Position::Last, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }

    #[test]
    fn horizontal_middle_zeroes_all() {
        let s = collapse_corners(dummy_style(), Position::Middle, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn only_keeps_every_corner() {
        let s = collapse_corners(dummy_style(), Position::Only, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }

    #[test]
    fn vertical_first_keeps_top_zeroes_bottom() {
        let s = collapse_corners(dummy_style(), Position::First, Orientation::Vertical);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn vertical_last_zeroes_top_keeps_bottom() {
        let s = collapse_corners(dummy_style(), Position::Last, Orientation::Vertical);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }
}
