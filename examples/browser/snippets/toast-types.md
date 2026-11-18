```rust
use iced::Task;
use icedcn::{
    Slot,
    widget::{button, row::Row, toast},
};
use toast::Kind::*;

#[derive(Clone)]
enum Message {
    CreateEvent,
}

fn types<'a>() -> Slot<'a, Message> {
    let kinds = [
        ("Default", Default, "Event has been created"),
        ("Success", Success, "Event has been created"),
        (
            "Info",
            Info,
            "Be at the area 10 minutes before the event time",
        ),
        (
            "Warning",
            Warning,
            "Event start time cannot be earlier than 8am",
        ),
        ("Error", Error, "Event has not been created"),
    ];
    let buttons = kinds.map(|(label, kind, title)| {
        toast::trigger(
            button(label).style(button::Outline),
            toast::toast(title).kind(kind),
        )
    });
    Row::with_children(buttons)
        .push(
            button("Promise")
                .style(button::Outline)
                .on_press(Message::CreateEvent)
                .into(),
        )
        .spacing(8)
        .wrap()
        .into()
}

// Return this task from update when handling CreateEvent.
fn creating(
    request: impl Future<Output = Result<String, String>> + Send + 'static,
) -> Task<Message> {
    toast::promise::<_, Message, _, _>(
        "Loading...",
        request,
        |name| format!("{name} has been created"),
        |error| error,
    )
}
```
