//! A native scrolling viewport with Nova's 10px hit target and rounded thumb.
use iced_widget::scrollable::{Status, Style};
use icedxdy_core::Theme;
use icedxdy_core::theme::scrollable::hidden;

use crate::Slot;
use crate::scrollable::{Direction, Scrollable, Scrollbar};

/// Nova's vertical scroll area. Set a height to constrain its viewport.
pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Scrollable<'a, Message> {
    Scrollable::new(content)
        .direction(Direction::Vertical(bar()))
        .style(paint)
}

/// Use this bar in a horizontal or two-axis scroll area's direction.
pub fn bar() -> Scrollbar {
    Scrollbar::default().width(10).scroller_width(7)
}

fn paint(theme: &Theme, status: Status) -> Style {
    let mut style = hidden(theme, status);
    if !matches!(status, Status::Active { .. }) {
        for rail in [&mut style.vertical_rail, &mut style.horizontal_rail] {
            rail.scroller.background = iced_core::Color::from(theme.neutral.border).into();
            rail.scroller.border.radius = 4.into();
        }
    }
    style
}
