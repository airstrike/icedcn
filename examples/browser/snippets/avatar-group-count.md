```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn preview(
    portrait: image::Handle,
    maxleiter: image::Handle,
    evilrabbit: image::Handle,
) -> avatar::group::Group<'static, ()> {
    avatar::group([
        avatar(Some(portrait)).fallback("AC"),
        avatar(Some(maxleiter)).fallback("LR"),
        avatar(Some(evilrabbit)).fallback("ER"),
    ])
    .count("+3")
}
```
