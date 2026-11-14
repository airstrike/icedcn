Use the `className` prop to add custom styles to the badge such as custom colors, sizes, etc.

In Rust, use `.badge_style(...)` to customize its surface and text color; badge geometry follows `.size(...)` on its Avatar.

```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::theme::avatar::badge_style;
use icedxdy::theme::container;
use icedxdy::widget::avatar;

fn presence(theme: &Theme) -> container::Style {
    let mut style = badge_style(theme);
    let color = if theme.neutral.base.l < 0.5 {
        iced::Color::from_rgb8(1, 102, 48)
    } else {
        iced::Color::from_rgb8(0, 166, 62)
    };
    style.background = Some(icedxdy::Color::from_srgb(color).into());
    style.border.width = 0.0;
    style
}

fn preview(shadcn: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar::badge(avatar(Some(shadcn)).fallback("CN"), avatar::dot())
        .badge_style(presence)
        .into()
}
```
