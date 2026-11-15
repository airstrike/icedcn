## Usage

Pass a trigger widget and a hint to `tooltip`. A focusable trigger opens its hint on keyboard focus. Escape dismisses the hint and keeps focus on the trigger.

```rust
use icedcn::{
    Slot,
    widget::{button, tooltip},
};

pub fn view() -> Slot<'static, ()> {
    tooltip(button("Hover").style(button::Outline), "Add to library").into()
}
```
