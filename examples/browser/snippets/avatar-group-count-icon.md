```rust
use crate::icon;
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(
    portrait: image::Handle,
    hecrj: image::Handle,
    iced_logo: image::Handle,
) -> avatar::group::Group<'a, ()> {
    avatar::group![
        avatar(portrait).fallback("AT"),
        avatar(hecrj).fallback("HR"),
        avatar(iced_logo).fallback("IC"),
    ]
    .count(icon::plus().size(16).line_height(1.0))
}
```
