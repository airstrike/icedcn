```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;

use iced::widget::{Svg, svg};

fn plus(size: f32) -> Svg<'static, Theme> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5v14"/></svg>"#))
        .width(size)
        .height(size)
}

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
    .count(plus(16.0).style(|theme: &Theme, _| svg::Style {
        color: Some(theme.muted.foreground.into()),
    }))
    .into()
}
```
