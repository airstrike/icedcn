```rust
use iced::widget::image;
use icedcn::widget::avatar;
use icedcn::widget::row;

fn preview(portrait: image::Handle) -> row::Row<'static, ()> {
    row![
        avatar(Some(portrait.clone())).fallback("AC").size(avatar::Sm),
        avatar(Some(portrait.clone())).fallback("AC"),
        avatar(Some(portrait)).fallback("AC").size(avatar::Lg),
    ]
    .spacing(8)
    .wrap()
    .align_y(iced::Center)
}
```
