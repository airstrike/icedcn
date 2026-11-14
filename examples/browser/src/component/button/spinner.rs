use icedxdy::Slot;
use icedxdy::widget::{button, row, spinner};

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    row![
        button![spinner().circle(), "Generating"]
            .style(button::Outline)
            .icon_start()
            .disabled(true),
        button!["Downloading", spinner().circle()]
            .style(button::Secondary)
            .icon_start()
            .disabled(true),
    ]
    .spacing(8)
    .into()
}
