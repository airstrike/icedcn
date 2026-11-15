## Compose widgets

Use lowercase constructors for required content and builder methods for optional properties. Variants go through `.style(...)`. Rows and columns accept `Slot` children; their macros make mixed child types convenient.

```rust
use icedcn::Slot;
use icedcn::widget::{button, column, field, input, text};

pub fn profile<'a>() -> Slot<'a, ()> {
    column![
        text::h3("Profile"),
        field(input("Display name")).label("Name"),
        button("Save").style(button::Outline),
    ]
    .spacing(12)
    .into()
}
```

A button without an `on_press` handler remains enabled and publishes no application message. Add a handler when the application has an action to perform. Use `.disabled(true)` to disable a control explicitly.

Parents pass inherited properties such as disabled state, text size, and weight through `Compose`. Prefer the library's `field`, `row`, and `column` helpers when arranging its controls so this inheritance reaches their children.
