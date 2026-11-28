```rust
use crate::icon;
use iced::Fill;
use icedcn::View;
use icedcn::widget::input_group::addon;
use icedcn::widget::{container, field, input, input_group};

fn inline_start<'a, M: Clone + 'a>(size: f32) -> View<'a, M> {
    container(
        field(
            input_group(input("Search...").text_size(size))
                .addon(addon(icon::search().size(16).line_height(1.0))),
        )
        .label("Input")
        .description("Icon positioned at the start."),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}
```
