```rust
use crate::icon;
use iced::Fill;
use icedcn::View;
use icedcn::widget::{button, input};

pub fn view<'a>() -> View<'a> {
    button::group![
        input("Search...").width(Fill),
        button(icon::search().size(16).line_height(1.0)).style(button::Outline)
    ]
    .width(Fill.max(240.))
    .into()
}
```
