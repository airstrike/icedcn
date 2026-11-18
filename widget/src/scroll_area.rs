//! A native scrolling viewport with Nova's 10px hit target and rounded thumb.
use iced_widget::scrollable::{Status, Style};
use icedcn_core::Theme;
use icedcn_core::theme::scrollable::hidden;

use crate::Slot;
use crate::scrollable::{Direction, Scrollable, Scrollbar, Visibility};

/// Nova's vertical scroll area. Set a height to constrain its viewport.
///
/// Like Radix's default `type="hover"`, its scrollbars show while the pointer is over it and
/// fade out [`HIDE_DELAY`](crate::scrollable::HIDE_DELAY) after it leaves. Change that with
/// `.visibility(..)` and `.hide_delay(..)`.
pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Scrollable<'a, Message> {
    Scrollable::new(content)
        .direction(Direction::Vertical(bar()))
        .visibility(Visibility::Hover)
        .style(paint)
}

/// Use this bar in a horizontal or two-axis scroll area's direction.
pub fn bar() -> Scrollbar {
    Scrollbar::default().width(10).scroller_width(7)
}

fn paint(theme: &Theme, status: Status) -> Style {
    let mut style = hidden(theme, status);
    for rail in [&mut style.vertical_rail, &mut style.horizontal_rail] {
        rail.scroller.background = iced_core::Color::from(theme.neutral.border).into();
        rail.scroller.border.radius = 4.into();
    }
    style
}
