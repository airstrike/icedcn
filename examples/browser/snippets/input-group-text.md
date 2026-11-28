```rust
use iced::Fill;
use icedcn::View;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{column, input, input_group, textarea};

fn text_addons<'a, M: Clone + 'a>(size: f32) -> View<'a, M> {
    column![
        input_group(input("0.00").text_size(size))
            .addon(addon(input_group::text("$")))
            .addon(addon(input_group::text("USD")).align(Align::InlineEnd)),
        input_group(input("example.com").text_size(size))
            .addon(addon(input_group::text("https://")))
            .addon(addon(input_group::text(".com")).align(Align::InlineEnd)),
        input_group(input("Enter your username").text_size(size))
            .addon(addon(input_group::text("@company.com")).align(Align::InlineEnd)),
        input_group(textarea("Enter your message").text_size(size)).addon(
            addon(
                input_group::text("120 characters left")
                    .size(12)
                    .line_height(iced::Pixels(16.)),
            )
            .align(Align::BlockEnd),
        ),
    ]
    .spacing(24)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
