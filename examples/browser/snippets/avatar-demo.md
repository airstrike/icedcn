```rust
use iced::widget::image;
use icedcn::widget::avatar::badge;
use icedcn::widget::{avatar, row};

fn view<'a>(
    portrait: image::Handle,
    hecrj: image::Handle,
    iced_logo: image::Handle,
    window_width: f32,
) -> row::Row<'a, ()> {
    row![
        avatar(portrait.clone()).fallback("AT"),
        avatar::badge(avatar(hecrj.clone()).fallback("HR"), avatar::dot(),)
            .badge_style(badge::presence),
        avatar::group![
            avatar(portrait).fallback("AT"),
            avatar(hecrj).fallback("HR"),
            avatar(iced_logo).fallback("IC"),
        ]
        .count("+3"),
    ]
    .spacing(if window_width >= 768.0 { 48 } else { 24 })
    .wrap()
    .align_y(iced::Center)
}
```
