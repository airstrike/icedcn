```rust
use crate::Message;
use iced::{Fill, Widget};
use icedcn::Slot;
use icedcn::widget::{button, button_group, column, field, popover, textarea};

pub fn view() -> Slot<'static, Message> {
    popover(
        |&p| {
            button_group![
                button![icon::bot().boxed(), "Copilot"].style(button::Outline),
                p.trigger(
                    button(icon::chevron_down().boxed())
                        .square()
                        .style(button::Outline)
                        .has_popup(true)
                )
            ]
            .into()
        },
        |_| {
            column![
                popover::header(
                    "Start a new task with Copilot",
                    "Describe your task in natural language."
                ),
                field(textarea("I need to..."))
                    .description("Copilot will open a pull request for review.")
            ]
            .spacing(12)
            .width(Fill)
            .into()
        },
    )
    .align(popover::Align::End)
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn bot<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8V4H8" /> <rect width="16" height="12" x="4" y="8" rx="2" /> <path d="M2 14h2" /> <path d="M20 14h2" /> <path d="M15 13v2" /> <path d="M9 13v2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn chevron_down<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m6 9 6 6 6-6" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
