## Usage

Build the trigger and the anchored content with two closures. The popover manages its open state.

```rust
use icedcn::Slot;
use icedcn::widget::{button, popover, text};

pub fn view<'a>() -> Slot<'a, ()> {
    popover(
        |p| p.trigger(button("Show details").has_popup(true)),
        |_| text("Project details").into(),
    )
    .into()
}
```
