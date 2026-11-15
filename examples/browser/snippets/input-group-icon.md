```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{column, input, input_group, row};

fn icons<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("Search...").text_size(size))
            .addon(addon(icon::search().size(16).line_height(1.0))),
        input_group(input("Enter your email").text_size(size))
            .addon(addon(icon::mail().size(16).line_height(1.0))),
        input_group(input("Card number").text_size(size))
            .addon(addon(icon::credit_card().size(16).line_height(1.0),))
            .addon(addon(icon::check().size(16).line_height(1.0)).align(Align::InlineEnd),),
        input_group(input("Card number").text_size(size)).addon(
            addon(
                row![
                    icon::star().size(16).line_height(1.0),
                    icon::info().size(16).line_height(1.0)
                ]
                .spacing(8),
            )
            .align(Align::InlineEnd),
        ),
    ]
    .spacing(24)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
