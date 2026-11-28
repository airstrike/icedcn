```rust
use crate::icon;
use iced::Fill;
use icedcn::View;
use icedcn::widget::input_group::Align;
use icedcn::widget::{button, input, input_group, tooltip};

pub fn view<'a>() -> View<'a> {
    button::group![
        button::group![
            button(icon::plus().size(16).line_height(1.0))
                .square()
                .style(button::Outline)
        ],
        button::group![
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
