```rust
use crate::icon;
use iced::Fill;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use icedcn::Size;
use icedcn::Slot;
use icedcn::widget::bubble::Secondary;
use icedcn::widget::{bubble, button, column, container, tooltip};

fn example<'a>() -> Slot<'a, ()> {
    frame(
        column![
            bubble("Did you remove the stale route?").style(Secondary),
            bubble("Yes, removed it from the registry.")
                .align(Right)
                .reactions(
                    tooltip(
                        button(icon::check().size(16).line_height(1.0))
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost),
                        "Read on Jan 5, 2026 at 4:32 PM"
                    ),
                    Right,
                    Bottom
                )
                .reaction_padding(0)
        ]
        .spacing(16)
        .width(Fill),
    )
}

fn frame(body: impl Into<Slot<'static, ()>>) -> Slot<'static, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
```
