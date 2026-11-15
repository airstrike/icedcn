Use `.badge_style(...)` to customize its surface and text color; badge geometry follows `.size(...)` on its Avatar.

```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::theme::avatar::badge_style;
use icedcn::theme::container;
use icedcn::widget::avatar;

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

fn preview(portrait: image::Handle) -> avatar::Avatar<'static, ()> {
    avatar::badge(avatar(Some(portrait)).fallback("AC"), avatar::dot())
        .badge_style(presence)
}
```
