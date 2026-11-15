```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::theme::{avatar::badge_style, container};
use icedcn::widget::{avatar, row};

fn presence(theme: &Theme) -> container::Style {
    let mut style = badge_style(theme);
    let color = if theme.neutral.base.l < 0.5 {
        iced::Color::from_rgb8(1, 102, 48)
    } else {
        iced::Color::from_rgb8(0, 166, 62)
    };
    style.background = Some(icedcn::Color::from_srgb(color).into());
    style.border.width = 0.0;
    style
}

fn preview(
    portrait: image::Handle,
    maxleiter: image::Handle,
    evilrabbit: image::Handle,
    window_width: f32,
) -> row::Row<'static, ()> {
    row![
        avatar(Some(portrait.clone())).fallback("AC"),
        avatar::badge(
            avatar(Some(evilrabbit.clone())).fallback("ER"),
            avatar::dot(),
        )
        .badge_style(presence),
        avatar::group([
            avatar(Some(portrait)).fallback("AC"),
            avatar(Some(maxleiter)).fallback("LR"),
            avatar(Some(evilrabbit)).fallback("ER"),
        ])
        .count("+3"),
    ]
    .spacing(if window_width >= 768.0 { 48 } else { 24 })
    .wrap()
    .align_y(iced::Center)
}
```
