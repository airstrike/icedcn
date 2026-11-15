```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, button_group, column, field, popover, textarea};

pub fn view<'a>() -> Slot<'a, ()> {
    popover(
        |&p| {
            button_group![
                button![icon::bot().size(16).line_height(1.0), "Copilot"].style(button::Outline),
                p.trigger(
                    button(icon::chevron_down().size(16).line_height(1.0))
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
```
