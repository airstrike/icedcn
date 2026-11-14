```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::Theme;
use icedcn::Size::Sm;
use icedcn::widget::button;

button![svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 18m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M7 6m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M17 6m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M7 8l0 8"></path><path d="M9 18h6a2 2 0 0 0 2 -2v-5"></path><path d="M14 14l3 -3l3 3"></path></svg>"#.as_bytes()))
            .width(14).height(14)
            .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
            .boxed(), "New Branch"].size(Sm).style(button::Outline)
```
