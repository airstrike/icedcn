```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;

fn preview(
    shadcn: image::Handle,
    maxleiter: image::Handle,
    evilrabbit: image::Handle,
) -> iced::Element<'static, (), Theme> {
    avatar::group([
        avatar(Some(shadcn)).fallback("CN"),
        avatar(Some(maxleiter)).fallback("LR"),
        avatar(Some(evilrabbit)).fallback("ER"),
    ])
    .into()
}
```
