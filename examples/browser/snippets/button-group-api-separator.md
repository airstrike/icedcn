### ButtonGroupSeparator

Visually divides buttons within a group. Its orientation is independent of the group's orientation.

| Builder | Type | Default |
| --- | --- | --- |
| `.orientation(...)` | `Horizontal` or `Vertical` | `Vertical` |

```rust
use icedcn::widget::button::group;
use icedcn::{Slot, widget::button};

pub fn actions<'a>() -> Slot<'a, ()> {
    button::group![button("Button 1"), group::separator(), button("Button 2")].into()
}
```
