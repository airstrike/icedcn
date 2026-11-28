```rust
use iced::Fill;
use icedcn::Size;
use icedcn::View;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{button, column, field, input, input_group, row, space, textarea};

fn block_end<'a, M: Clone + 'a>(size: f32) -> View<'a, M> {
    column![
        field(
            input_group(input("Enter amount").text_size(size))
                .addon(addon(input_group::text("USD")).align(Align::BlockEnd)),
        )
        .label("Input")
        .description("Footer positioned below the input."),
        field(
            input_group(textarea("Write a comment...").text_size(size)).addon(
                addon(
                    row![
                        input_group::text("0/280"),
                        space::horizontal(),
                        input_group::button("Post")
                            .size(Size::Sm)
                            .height(32)
                            .style(button::Default)
                    ]
                    .width(Fill),
                )
                .align(Align::BlockEnd),
            ),
        )
        .label("Textarea")
        .description("Footer positioned below the textarea."),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
