```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;
use icedxdy::widget::row;

fn preview(portrait: image::Handle) -> iced::Element<'static, (), Theme> {
    row![
        avatar(Some(portrait.clone())).fallback("AC").size(avatar::Sm),
        avatar(Some(portrait.clone())).fallback("AC"),
        avatar(Some(portrait)).fallback("AC").size(avatar::Lg),
    ]
    .spacing(8)
    .wrap()
    .align_y(iced::Center)
    .into()
}
```
