```rust
use iced::Fill;
use iced::alignment::Horizontal::{Left, Right};
use iced::alignment::Vertical::Bottom;
use icedcn::Slot;
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, column, container};

fn example<'a>() -> Slot<'a, ()> {
    frame(
        column![
            bubble("Can you tell me what's the issue?").style(Muted),
            bubble::group([
                bubble("You tell me!").align(Right).into(),
                bubble("It worked yesterday. You broke it!")
                    .align(Right)
                    .into(),
                bubble("Find the bug and fix it.")
                    .align(Right)
                    .reactions("👀", Left, Bottom)
                    .into()
            ]),
            bubble("Want me to diff yesterday's you against today's you? It's a bit embarrassing.")
                .style(Muted)
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
