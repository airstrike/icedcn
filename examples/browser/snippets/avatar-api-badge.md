### AvatarBadge

| Rust API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `avatar::badge(avatar, content)` | `Avatar`, `impl Into<Slot>` | no badge | Returns the Avatar with a retained badge, usable in a group. |
| `.badge_style(style)` | `Fn(&Theme) -> container::Style` | primary | Styles the badge while retaining its 2px outer background ring. |
| `avatar::dot()` | empty `Slot` | — | Presence-dot content. |

The badge disk is 8/10/12px for small/default/large. Small avatars hide the inner content. Compose-aware content inherits 8px; set stock iced icon width and height explicitly to 8px.

```rust
use iced::widget::{Svg, svg};
use icedcn::Theme;
use icedcn::widget::avatar;

fn plus(size: f32) -> Svg<'static, Theme> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5v14"/></svg>"#))
        .width(size)
        .height(size)
}
use icedcn::theme::avatar::badge_style;
use icedcn::theme::container;

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

fn preview() -> avatar::group::Group<'static, ()> {
    avatar::group([
        avatar::badge(avatar(None).fallback("AC"), avatar::dot()).badge_style(presence),
        avatar::badge(
            avatar(None).fallback("PP"),
            plus(8.0).style(|theme: &Theme, _| svg::Style {
                color: Some(theme.primary.foreground.into()),
            }),
        ),
    ])
}
```
