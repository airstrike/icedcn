### ButtonGroupText

Display text within a button group with `group::text(content)`. The content accepts `impl Into<Slot>`, including custom labels.

```rust
use icedcn::{Slot, widget::{button, button_group, input, text}};
use icedcn::widget::button::group;

pub fn actions() -> Slot<'static, ()> {
    button_group![group::text("Text"), button("Button")].into()
}

pub fn custom_label() -> Slot<'static, ()> {
    button_group![
        group::text(text::label("Name")),
        input("Type something here...").width(192),
    ].into()
}
```
