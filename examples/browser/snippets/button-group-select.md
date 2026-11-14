```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, button_group, input, row, select, text};

pub fn view(window_width: f32) -> Slot<'static, ()> {
    let narrow = window_width < 768.;
    button_group![
        button_group![
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
            .width(50.406)
            .min_width(96.),
            input("10.00")
                .text_size(if narrow { 16. } else { 14. })
                .width(Fill)
        ]
        .width(if narrow { 220. } else { 241.406 }),
        button_group![
            button(icon::arrow_right().size(16).line_height(1.0)).square().style(button::Outline)
        ]
    ]
    .into()
}
```
