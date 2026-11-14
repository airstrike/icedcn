```rust
use icedxdy::Slot;
use icedxdy::widget::{bubble, message, motion, scroller};
use icedxdy::widget::motion::Preset;

fn conversation<'a, Message: 'a>(
    history: &'a str,
    sent: Option<&'a str>,
    preset: Preset,
) -> Slot<'a, Message> {
    let previous = scroller::item(1, message(bubble(history)));
    let incoming = sent.map(|text| {
        scroller::item(2, message(motion(bubble(text)).preset(preset))).anchor(true)
    });

    scroller([previous].into_iter().chain(incoming)).into()
}
```
