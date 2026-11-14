```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::{button, button_group, input, input_group, tooltip};
use icedcn::widget::input_group::Align;

pub fn view(window_width: f32) -> Slot<'static, ()> {
    let narrow = window_width < 768.;
    button_group![
        button_group![
            button(icon::plus().size(16).line_height(1.0)).square().style(button::Outline)
        ],
        button_group![
            input_group(input("Send a message...").text_size(if narrow { 16. } else { 14. }))
                .addon(
                    input_group::addon(tooltip(
                        icon::audio_lines().size(16).line_height(1.0),
                        "Voice Mode"
                    ))
                    .align(Align::InlineEnd)
                )
        ]
        .width(if narrow { 220. } else { 212. })
    ]
    .into()
}
```
