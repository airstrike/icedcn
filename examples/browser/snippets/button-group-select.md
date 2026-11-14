```rust
use crate::Message;
use iced::{Fill, Widget};
use icedcn::Slot;
use icedcn::widget::{button, button_group, input, row, select, text};

pub fn view(window_width: f32) -> Slot<'static, Message> {
    let narrow = window_width < 768.;
    button_group![
        button_group![
            select(
                [("$", "US Dollar"), ("€", "Euro"), ("£", "British Pound")]
                    .map(|(value, label)| select::item(value, label))
            )
            .default_value("$")
            .format_value(|value, _| value.to_owned())
            .font(iced::Font::MONOSPACE)
            .format_item(|value, label| row![
                text::small(value.to_owned()),
                text::muted(label.to_owned())
            ]
            .spacing(4))
            .width(50.406)
            .min_width(96.)
            .icon_content(icon::chevron_down),
            input("10.00")
                .text_size(if narrow { 16. } else { 14. })
                .width(Fill)
        ]
        .width(if narrow { 220. } else { 241.406 }),
        button_group![
            button(icon::arrow_right().boxed())
                .square()
                .style(button::Outline)
        ]
    ]
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn arrow_right<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="m12 5 7 7-7 7" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn chevron_down<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m6 9 6 6 6-6" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
