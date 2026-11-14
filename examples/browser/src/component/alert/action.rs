use icedxdy::Size;
use icedxdy::Slot;
use icedxdy::widget::{alert, button, text};

use crate::Message;

pub fn view<'a>(wrapping: text::Wrapping) -> Slot<'a, Message> {
    alert()
        .description_wrapping(wrapping)
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .action(button("Enable").size(Size::Xs))
        .max_width(448)
        .into()
}
