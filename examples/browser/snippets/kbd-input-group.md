```rust
use crate::icon;
use icedcn::widget::input_group::Align;
use icedcn::widget::{container, input, input_group, kbd};

container(
    input_group(input("Search..."))
        .addon(input_group::addon(icon::search()))
        .addon(input_group::addon(kbd::group![kbd("⌘"), kbd("K")]).align(Align::InlineEnd)),
)
.max_width(320)
.width(iced::Fill)
```
