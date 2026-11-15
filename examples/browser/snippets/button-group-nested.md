```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::input_group::Align;
use icedcn::widget::{button, button_group, input, input_group, tooltip};

pub fn view<'a>() -> Slot<'a, ()> {
    button_group![
        button_group![
            button(icon::plus().size(16).line_height(1.0))
                .square()
                .style(button::Outline)
        ],
        button_group![
            input_group(input("Send a message...")).addon(
                input_group::addon(tooltip(
                    icon::audio_lines().size(16).line_height(1.0),
                    "Voice Mode"
                ))
                .align(Align::InlineEnd)
            )
        ]
        .width(Fill)
    ]
    .width(Fill.max(256.))
    .into()
}
```
