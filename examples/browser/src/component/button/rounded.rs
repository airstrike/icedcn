use crate::{Message, icon};
use icedcn::Slot;
use icedcn::widget::button;

pub fn view<'a>() -> Slot<'a, Message> {
    button(icon::arrow_up().size(16))
        .square()
        .rounded()
        .style(button::Outline)
        .into()
}
