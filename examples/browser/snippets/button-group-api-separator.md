### ButtonGroupSeparator

Visually divides buttons within a group. Its orientation is independent of the group's orientation.

| Builder | Type | Default |
| --- | --- | --- |
| `.orientation(...)` | `Horizontal` or `Vertical` | `Vertical` |

```rust
use icedcn::{Slot, widget::{button, button_group}};
use icedcn::widget::button::group;

pub fn actions() -> Slot<'static, ()> {
    button_group![button("Button 1"), group::separator(), button("Button 2")].into()
}
```
