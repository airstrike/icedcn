```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, input, row, select, text};

pub fn view<'a>() -> Slot<'a, ()> {
    button::group![
        button::group![
            select(
                [("$", "US Dollar"), ("€", "Euro"), ("£", "British Pound")]
                    .map(|(value, label)| select::item(value, label))
            )
            .default_value("$")
            .format_value(|value, _| value.to_owned())
            .font(iced::Font::MONOSPACE)
            .format_item(|value, label| row![
                text::small(value.to_owned()),
                text::muted(label.to_owned())
            ]
            .spacing(4))
            .width(50)
            .min_width(96.),
            input("10.00").width(Fill)
        ]
        .width(Fill),
        button::group![
            button(icon::arrow_right().size(16).line_height(1.0))
                .square()
                .style(button::Outline)
        ]
    ]
    .width(Fill.max(280.))
    .into()
}
```
