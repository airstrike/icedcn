use icedxdy::Slot;
use icedxdy::widget::{button, row, spinner};

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    row![
        button![spinner().bar_height(4.0 / 3.0), "Generating"]
            .style(button::Outline)
            .icon_start()
            .disabled(true),
        button!["Downloading", spinner().bar_height(4.0 / 3.0)]
            .style(button::Secondary)
            .icon_start()
            .disabled(true),
    ]
    .spacing(8)
    .into()
}
