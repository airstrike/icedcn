```rust
use crate::icon;
use icedcn::widget::{button, column};
use icedcn::{Size, Slot};

pub fn view<'a>() -> Slot<'a, ()> {
    let sizes = [
        (Size::Sm, "Small"),
        (Size::Default, "Default"),
        (Size::Lg, "Large"),
    ];
    column(sizes.map(|(size, first)| {
        let action = |label| button(label).size(size).style(button::Outline);
        button::group![
            action(first),
            action("Button"),
            action("Group"),
            button(icon::plus().size(16).line_height(1.0))
                .size(size)
                .square()
                .style(button::Outline),
        ]
        .into()
    }))
    .spacing(32)
    .align_x(iced::Left)
    .into()
}
```
