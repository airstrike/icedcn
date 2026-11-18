```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{container, field, input, input_group};

fn inline_end<'a, M: Clone + 'a>(size: f32) -> Slot<'a, M> {
    container(
        field(
            input_group(input("Enter password").text_size(size).secure(true))
                .addon(addon(icon::eye_off().size(16).line_height(1.0)).align(Align::InlineEnd)),
        )
        .label("Input")
        .description("Icon positioned at the end."),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}
```
