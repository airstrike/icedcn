```rust
use iced::alignment::Horizontal::Right;
use icedcn::Slot;
use icedcn::widget::{bubble, message, motion, scroller};
use icedcn::widget::motion::Preset;

pub fn conversation<'a, Message: Clone + 'a>(
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
