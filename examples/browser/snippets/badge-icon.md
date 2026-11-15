```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::Theme;
use icedcn::widget::{badge, row};

fn example() -> row::Row<'static, ()> {
    row![
        badge![svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 12 2 2 4-4"/></svg>"#.as_bytes()))
                .width(12)
                .height(12)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.secondary.foreground.into()) })
                .boxed(), "Verified"]
            .icon_start()
            .style(badge::Secondary),
        badge!["Bookmark", svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/></svg>"#.as_bytes()))
                .width(12)
                .height(12)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                .boxed()]
            .icon_end()
            .style(badge::Outline),
    ]
    .spacing(8)
    .wrap()
}
```
