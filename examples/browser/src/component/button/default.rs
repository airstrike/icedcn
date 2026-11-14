use icedcn::Slot;
use icedcn::widget::button;

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button!["Button"].into()
}
