```rust
use iced::alignment::Horizontal::Right;
use icedxdy::Slot;
use icedxdy::widget::{bubble, message, motion, scroller};
use icedxdy::widget::motion::Preset;

fn conversation<'a, Message: 'a>(
    history: &'a str,
    sent: Option<&'a str>,
    preset: Preset,
) -> Slot<'a, Message> {
    let previous = scroller::item(1, message(bubble(history).style(bubble::Ghost)));
    let incoming = sent.map(|text| {
        let user = message(bubble(text).style(bubble::Muted)).align(Right);
        let entrance = motion(user).preset(preset).theme(bubble::fade_muted_on_card);
        scroller::item(2, entrance).anchor(true)
    });

    scroller([previous].into_iter().chain(incoming)).into()
}
```
