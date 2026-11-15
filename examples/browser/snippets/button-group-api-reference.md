### ButtonGroup

A container that groups related buttons together with consistent styling. Construct it with `button_group![...]`, or `group::group` over an iterator of `Slot`s.

| Builder | Type | Default |
| --- | --- | --- |
| `.orientation(...)` | `Horizontal` or `Vertical` | `Horizontal` |
| `.width(...)` | `impl Into<Length>` | `Length::Shrink` |

```rust
use icedcn::{
    Slot,
    widget::{button, button_group},
};

pub fn actions<'a>() -> Slot<'a, ()> {
    button_group![button("Button 1"), button("Button 2")].into()
}

pub fn nested<'a>() -> Slot<'a, ()> {
    button_group![
        button_group![button("Button 1")],
        button_group![button("Button 2")],
    ]
    .into()
}
```

Nest multiple button groups to create complex layouts with spacing. See Nested for more details. Buttons, inputs, selects and input groups consume the joining context through `Compose`. Tooltips and popup triggers preserve that context; popup content starts a separate surface. Inherited size and disabled state continue through nested groups.
