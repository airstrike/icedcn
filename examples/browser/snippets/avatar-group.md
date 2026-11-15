```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn preview(
    portrait: image::Handle,
    hecrj: image::Handle,
    iced_logo: image::Handle,
) -> avatar::group::Group<'static, ()> {
    avatar::group([
        avatar(Some(portrait)).fallback("AT"),
        avatar(Some(hecrj)).fallback("HR"),
        avatar(Some(iced_logo)).fallback("IC"),
    ])
}
```
