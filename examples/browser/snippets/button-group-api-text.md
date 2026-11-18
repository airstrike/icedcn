### ButtonGroupText

Display text within a button group with `group::text(content)`. The content accepts `impl Into<Slot>`, including custom labels.

```rust
use icedcn::widget::button::group;
use icedcn::{
    Slot,
    widget::{button, input, text},
};

pub fn actions<'a>() -> Slot<'a, ()> {
    button::group![group::text("Text"), button("Button")].into()
}

pub fn custom_label<'a>() -> Slot<'a, ()> {
    button::group![
        group::text(text::label("Name")),
        input("Type something here...").width(192),
    ]
    .into()
}
```
