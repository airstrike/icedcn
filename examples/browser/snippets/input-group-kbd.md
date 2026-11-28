```rust
use crate::icon;
use iced::Fill;
use icedcn::View;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{container, input, input_group, kbd};

fn shortcut<'a, M: Clone + 'a>(size: f32) -> View<'a, M> {
    container(
        input_group(input("Search...").text_size(size))
            .addon(addon(icon::search().size(16).line_height(1.0)))
            .addon(addon(kbd("⌘K")).align(Align::InlineEnd)),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}
```
