```rust
use iced::widget::svg;
use icedcn::{Size, Theme};
use icedcn::widget::button;

let branch = svg(svg::Handle::from_memory(include_bytes!("git-branch.svg").as_slice()))
    .width(14)
    .height(14)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) });

button(branch).push("New Branch").size(Size::Sm).style(button::Outline)
```
