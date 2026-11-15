```rust
use icedcn::widget::{field, radio_group};
use icedcn::{Horizontal, Slot};

fn plans<M: Clone + 'static>() -> Slot<'static, M> {
    let plans = [
        ("Plus", "For individuals and small teams."),
        ("Pro", "For growing businesses."),
        ("Enterprise", "For large teams and enterprises."),
    ];
    radio_group(move |g| {
        field::group(plans.map(|(name, detail)| {
            field(g.item(name))
                .orientation(Horizontal)
                .control_last()
                .label(name)
                .description(detail)
                .card()
                .into()
        }))
        .spacing(8)
        .width(384)
        .into()
    })
    .default_value("Plus")
    .into()
}
```
