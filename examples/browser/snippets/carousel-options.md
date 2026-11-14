## Options

Use `.align(...)` to position the selected slide and `.looping(true)` to wrap navigation at the ends of the track.

```rust
use iced::Alignment;
use icedcn::Slot;
use icedcn::widget::{carousel, container, text};

pub fn view<Message: Clone + 'static>() -> Slot<'static, Message> {
    carousel(5, |index| container(text::p((index + 1).to_string())).height(160))
        .width(320)
        .align(Alignment::Start)
        .looping(true)
        .into()
}
```

`.default_value(index)` seeds the first snap. `.value(index)` makes selection controlled: store the complete index sent by `.on_value_change` in your model. Non-looping tracks contain both endpoints; looping falls back to containment when all slides fit.
