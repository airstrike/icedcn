```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use icedcn::Slot;
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, column, container, row};

fn example<'a>() -> Slot<'a, ()> {
    frame(
        column![
            bubble("Hey there! what's up?").align(Right),
            bubble::group([
                bubble("Hey! Want to see chat bubbles?").style(Muted).into(),
                bubble(
                    "I can group messages, switch sides, and keep the whole thread easy to scan."
                )
                .style(Muted)
                .reactions("👍", Right, Bottom)
                .into()
            ]),
            bubble("Sure. Hit me with your best demo.").align(Right),
            bubble("Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand.")
                .style(Muted)
                .reactions(row!["👍", "🔥", "👀", "+2"].spacing(4), Right, Bottom)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn frame<'a>(body: impl Into<Slot<'a, ()>>) -> Slot<'a, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
```
