```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedcn::View;
use icedcn::widget::bubble::{Muted, Tinted};
use icedcn::widget::{bubble, column, container, toast, toaster};

fn example<'a>() -> View<'a> {
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

fn frame<'a>(body: impl Into<View<'a>>) -> View<'a> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}

fn suggestion<'a>(label: &'a str, notice: &'a str) -> View<'a> {
    toast::trigger(
        bubble(label).style(Tinted).align(Right).interactive(true),
        toast::toast(notice),
    )
}
```
