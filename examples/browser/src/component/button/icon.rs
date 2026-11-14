use icedxdy::Slot;
use icedxdy::widget::button;

use super::icons;
use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button(icons::circle_fading_arrow_up(16.0))
        .square()
        .style(button::Outline)
        .into()
}
