```rust
use crate::Message;
use iced::Widget;
use icedcn::Slot;
use icedcn::widget::{button, button_group};
use icedcn::widget::button::group;

pub fn view() -> Slot<'static, Message> {
    button_group![
        button(icon::plus().boxed()).square().style(button::Outline),
        button(icon::minus().boxed())
            .square()
            .style(button::Outline),
    ]
    .orientation(group::Vertical)
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn minus<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn plus<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
