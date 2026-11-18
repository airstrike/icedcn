## Usage

Use `.default_value(..)` to let the group own its selection. Use `.value(..)` and `.on_value_change(..)` when another part of your app needs the selected value.

```rust
use icedcn::widget::{field, radio_group};
use icedcn::{Horizontal, Slot};

fn delivery<'a, M: Clone + 'static>(
    value: String,
    changed: impl Fn(String) -> M + 'a,
) -> Slot<'a, M> {
    radio_group(|g| {
        field::group(["Email", "SMS"].map(|name| {
            field(g.item(name))
                .orientation(Horizontal)
                .label(name)
                .into()
        }))
        .spacing(8)
        .into()
    })
    .value(value)
    .on_value_change(changed)
    .into()
}
```
