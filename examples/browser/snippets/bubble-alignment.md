```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedcn::View;
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, column, container};

fn example<'a>() -> View<'a> {
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

fn frame<'a>(body: impl Into<View<'a>>) -> View<'a> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
```
