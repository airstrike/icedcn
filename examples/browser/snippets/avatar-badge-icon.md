```rust
use crate::icon;
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(iced_logo: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar::badge(
        avatar(iced_logo).fallback("IC"),
        icon::plus().size(8).line_height(1.0),
    )
}
```
