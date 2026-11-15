```rust
use iced::Center;
use icedcn::widget::{button, kbd, row, tooltip};
use icedcn::{Size, Slot};

pub fn save_hint<'a>(save_icon: impl Into<Slot<'a, ()>>) -> Slot<'a, ()> {
    tooltip(
        button(save_icon)
            .size(Size::Sm)
            .square()
            .style(button::Outline),
        row!["Save Changes", kbd("S").style(kbd::Tooltip)]
            .spacing(6)
            .align_y(Center),
    )
    .padding([6, 6, 6, 12])
    .into()
}
```

Supply your application's Save icon as `save_icon`.
