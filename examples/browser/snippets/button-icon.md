```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::Theme;
use icedcn::widget::button;

button(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2a10 10 0 0 1 7.38 16.75"></path><path d="m16 12-4-4-4 4"></path><path d="M12 16V8"></path><path d="M2.5 8.875a10 10 0 0 0-.5 3"></path><path d="M2.83 16a10 10 0 0 0 2.43 3.4"></path><path d="M4.636 5.235a10 10 0 0 1 .891-.857"></path><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"></path></svg>"#.as_bytes()))
            .width(16).height(16)
            .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
            .boxed()).square().style(button::Outline)
```
