## Usage

```rust
use icedcn::{Slot, widget::button};

pub fn view<'a>() -> Slot<'a, ()> {
    button::group![button("Button 1"), button("Button 2")].into()
}
```

The group sizes itself to its controls. For an input that should share the available space, give the input `Fill` and bound its group with `.width(Fill.max(240.))`; no window-size state is needed. Icon examples use the shared [icon setup](icedcn://guide/installation#icons).
