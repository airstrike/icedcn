use icedcn::View;
use icedcn::widget::button;

use crate::Message;

pub fn view<'a>() -> View<'a, Message> {
    button("Button").into()
}
