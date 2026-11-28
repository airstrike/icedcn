```rust
use iced::Fill;
use icedcn::Size;
use icedcn::View;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{button, container, input_group, row, space, textarea};

fn custom<'a, M: Clone + 'a>(size: f32) -> View<'a, M> {
    container(
        input_group(textarea("Autoresize textarea...").text_size(size)).addon(
            addon(
                row![
                    space::horizontal(),
                    input_group::button("Submit")
                        .size(Size::Sm)
                        .height(32)
                        .style(button::Default)
                ]
                .width(Fill),
            )
            .align(Align::BlockEnd),
        ),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}
```
