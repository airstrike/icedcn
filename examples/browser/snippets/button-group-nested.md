```rust
use crate::Message;
use iced::Widget;
use icedcn::Slot;
use icedcn::widget::{button, button_group, container, input, input_group, tooltip};
use icedcn::widget::input_group::Align;

pub fn view(window_width: f32) -> Slot<'static, Message> {
    let narrow = window_width < 768.;
    button_group![
        button_group![button(icon::plus().boxed()).square().style(button::Outline)],
        button_group![
            input_group(input("Send a message...").text_size(if narrow { 16. } else { 14. }))
                .addon(
                    input_group::addon(tooltip(
                        container(icon::audio_lines().boxed()).padding([0, 0]),
                        "Voice Mode"
                    ))
                    .align(Align::InlineEnd)
                )
        ]
        .width(if narrow { 220. } else { 212. })
    ]
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn audio_lines<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 10v3" /> <path d="M6 6v11" /> <path d="M10 3v18" /> <path d="M14 8v7" /> <path d="M18 5v13" /> <path d="M22 10v3" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn plus<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
