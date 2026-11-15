```rust
use icedcn::{Size, Slot};
use icedcn::widget::{button, popover, row, text};
use icedcn::widget::popover::Align;

pub fn view<'a>() -> Slot<'a, ()> {
    let panel = |label: &'static str, alignment, content: &'static str| {
        popover(
            move |p| p.trigger(button(label).size(Size::Sm).style(button::Outline).has_popup(true)),
            move |_| text(content).into(),
        )
        .align(alignment)
        .width(160)
    };

    row![
        panel("Start", Align::Start, "Aligned to start"),
        panel("Center", Align::Center, "Aligned to center"),
        panel("End", Align::End, "Aligned to end"),
    ]
    .spacing(24)
    .into()
}
```
