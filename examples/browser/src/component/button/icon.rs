use crate::{Message, icon};
use icedcn::View;
use icedcn::widget::button;

pub fn view<'a>() -> View<'a, Message> {
    button(icon::circle_fading_arrow_up().size(16))
        .square()
        .style(button::Outline)
        .into()
}
