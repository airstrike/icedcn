```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedcn::Slot;
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, column, container};

fn example() -> Slot<'static, ()> {
    frame(
        column![
            bubble("This bubble is aligned to the start. This is the default alignment.")
                .style(Muted),
            bubble("This bubble is aligned to the end. Use this for user messages.").align(Right)
        ]
        .spacing(32)
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
