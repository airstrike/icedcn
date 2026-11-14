```rust
use crate::Message;
use iced::Widget;
use icedcn::Slot;
use icedcn::widget::{button, button_group, input};

pub fn view(window_width: f32) -> Slot<'static, Message> {
    let narrow = window_width < 768.;
    button_group![
        input("Search...")
            .text_size(if narrow { 16. } else { 14. })
            .width(if narrow { 216. } else { 192. }),
        button(icon::search().boxed()).style(button::Outline)
    ]
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn search<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="11" cy="11" r="8" /> <path d="m21 21-4.3-4.3" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
