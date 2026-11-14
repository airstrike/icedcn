```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;
use icedxdy::widget::row;

fn preview(shadcn: image::Handle) -> iced::Element<'static, (), Theme> {
    row![
        avatar(Some(shadcn.clone())).fallback("CN").size(avatar::Sm),
        avatar(Some(shadcn.clone())).fallback("CN"),
        avatar(Some(shadcn)).fallback("CN").size(avatar::Lg),
    ]
    .spacing(8)
    .wrap()
    .align_y(iced::Center)
    .into()
}
```
