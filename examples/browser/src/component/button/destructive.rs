use icedcn::Slot;
use icedcn::widget::button;

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Destructive").style(button::Destructive).into()
}
