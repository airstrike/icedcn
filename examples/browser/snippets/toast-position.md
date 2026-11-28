```rust
use iced::Center;
use icedcn::{
    View,
    widget::{button, row::Row, toast},
};
use toast::Position::*;

fn positions<'a>() -> View<'a> {
    let places = [
        ("Top Left", TopLeft),
        ("Top Center", TopCenter),
        ("Top Right", TopRight),
        ("Bottom Left", BottomLeft),
        ("Bottom Center", BottomCenter),
        ("Bottom Right", BottomRight),
    ];
    Row::with_children(places.map(|(label, position)| {
        toast::trigger(
            button(label).style(button::Outline),
            toast::toast("Event has been created").position(position),
        )
    }))
    .spacing(8)
    .wrap()
    .align_x(Center)
    .into()
}
```
