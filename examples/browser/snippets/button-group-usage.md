## Usage

```rust
use icedcn::{Slot, widget::{button, button_group}};

pub fn view() -> Slot<'static, ()> {
    button_group![button("Button 1"), button("Button 2")].into()
}
```
