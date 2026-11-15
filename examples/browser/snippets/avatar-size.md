```rust
use iced::widget::image;
use icedcn::widget::avatar;
use icedcn::widget::row;

fn view<'a>(portrait: image::Handle) -> row::Row<'a, ()> {
    row![
        avatar(portrait.clone()).fallback("AT").size(avatar::Sm),
        avatar(portrait.clone()).fallback("AT"),
        avatar(portrait).fallback("AT").size(avatar::Lg),
    ]
    .spacing(8)
    .wrap()
    .align_y(iced::Center)
}
```
