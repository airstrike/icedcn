## Accessibility

Use Tab to navigate between the buttons in the group. Each enabled control keeps its own keyboard focus and activation behavior; grouping does not introduce arrow-key selection.

Give related actions clear text labels and provide descriptive tooltips for icon-only controls.

```rust
use icedcn::{View, widget::button};

pub fn actions<'a>() -> View<'a> {
    button::group![button("Button 1"), button("Button 2")].into()
}
```
