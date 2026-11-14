```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::widget::avatar;

fn preview(
    portrait: image::Handle,
    maxleiter: image::Handle,
    evilrabbit: image::Handle,
) -> iced::Element<'static, (), Theme> {
    avatar::group([
        avatar(Some(portrait)).fallback("AC"),
        avatar(Some(maxleiter)).fallback("LR"),
        avatar(Some(evilrabbit)).fallback("ER"),
    ])
    .into()
}
```
