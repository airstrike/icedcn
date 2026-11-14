```rust
use crate::Message;
use iced::Widget;
use icedcn::Slot;
use icedcn::Size::{Lg, Sm};
use icedcn::widget::{button, button_group, column};

pub fn view() -> Slot<'static, Message> {
    column([
        button_group![
            button("Small").style(button::Outline).size(Sm),
            button("Button").style(button::Outline).size(Sm),
            button("Group").style(button::Outline).size(Sm),
            button(icon::plus().boxed())
                .square()
                .style(button::Outline)
                .size(Sm),
        ]
        .into(),
        button_group![
            button("Default").style(button::Outline),
            button("Button").style(button::Outline),
            button("Group").style(button::Outline),
            button(icon::plus().boxed()).square().style(button::Outline),
        ]
        .into(),
        button_group![
            button("Large").style(button::Outline).size(Lg),
            button("Button").style(button::Outline).size(Lg),
            button("Group").style(button::Outline).size(Lg),
            button(icon::plus().boxed())
                .square()
                .style(button::Outline)
                .size(Lg),
        ]
        .into(),
    ])
    .spacing(32)
    .align_x(iced::Left)
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn plus<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
