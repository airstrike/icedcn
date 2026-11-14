```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::Theme;
use icedcn::widget::{button, row};

row![
    button("Button").style(button::Outline),
    button(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 12 7-7 7 7"></path><path d="M12 19V5"></path></svg>"#.as_bytes()))
            .width(16).height(16)
            .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
            .boxed()).square().style(button::Outline),
].spacing(8)
```
