## Usage

A textarea keeps its own text and grows as you type. Use `.default_value("Draft")` to start with text, or give the application ownership with `.value(..)` and `.on_input(..)`.

```rust
use icedcn::Slot;
use icedcn::widget::textarea;

pub fn message(value: &str) -> Slot<'static, String> {
    textarea("Type your message here.")
        .value(value)
        .on_input(|edited| edited)
        .into()
}
```

The minimum height is 64 pixels. Set it with `.min_height(..)`, change the font size with `.text_size(..)`, or hide the vertical resize handle with `.resizable(false)`.
