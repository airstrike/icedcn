```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{column, field, input, input_group, row, space, textarea};

fn block_start<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        field(
            input_group(input("Enter your name").text_size(size))
                .addon(addon(input_group::text("Full Name")).align(Align::BlockStart)),
        )
        .label("Input")
        .description("Header positioned above the input."),
        field(
            input_group(
                textarea("console.log('Hello, world!');")
                    .text_size(14.)
                    .font(iced::Font::MONOSPACE),
            )
            .addon(
                addon(
                    row![
                        icon::file_code().size(16).line_height(1.0),
                        input_group::text("script.js").font(iced::Font::MONOSPACE),
                        space::horizontal(),
                        input_group::button(icon::copy().size(16).line_height(1.0)).square()
                    ]
                    .spacing(8)
                    .width(Fill),
                )
                .align(Align::BlockStart),
            ),
        )
        .label("Textarea")
        .description("Header positioned above the textarea."),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
