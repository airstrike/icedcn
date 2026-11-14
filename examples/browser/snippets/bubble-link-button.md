```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedxdy::Slot;
use icedxdy::widget::bubble::{Muted, Tinted};
use icedxdy::widget::{bubble, column, container, toast, toaster};

fn example() -> Slot<'static, ()> {
    toaster(frame(
        column![
            bubble("How can I help you today?").style(Muted),
            bubble::group([
                suggestion("I forgot my password", "You clicked forgot password"),
                suggestion(
                    "I need help with my subscription",
                    "You clicked help with subscription"
                ),
                suggestion(
                    "Something else. Talk to a human.",
                    "You clicked something else. Talk to a human."
                ),
            ])
        ]
        .spacing(32)
        .width(Fill),
    ))
    .into()
}

fn frame(body: impl Into<Slot<'static, ()>>) -> Slot<'static, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}

fn suggestion(label: &'static str, notice: &'static str) -> Slot<'static, ()> {
    toast::trigger(
        bubble(label).style(Tinted).align(Right).interactive(true),
        toast::toast(notice),
    )
}
```
