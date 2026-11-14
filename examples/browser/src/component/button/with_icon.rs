use icedcn::Size::*;
use icedcn::Slot;
use icedcn::widget::button;

use super::icons;
use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button![icons::git_branch(14.0), "New Branch"]
        .size(Sm)
        .style(button::Outline)
        .into()
}
