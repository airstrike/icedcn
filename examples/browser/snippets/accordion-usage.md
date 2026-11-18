## Usage

Use tuples for a fixed list. Add item properties through the builder when needed.

```rust
use icedcn::widget::accordion;

accordion![
    (
        "Is it accessible?",
        "Use Tab to focus a header, then Enter or Space to toggle it."
    ),
    accordion::item(
        "Premium features",
        "Upgrade your plan to access these features."
    )
    .disabled(true),
]
.default_value(0)
```

When the application owns the selection, pass its current `Option<usize>` and handle the proposed next value.

```rust
use icedcn::widget::{Accordion, accordion};

fn questions<'a, M: 'a>(
    selected: Option<usize>,
    changed: impl Fn(Option<usize>) -> M + 'a,
) -> Accordion<'a, M> {
    accordion![
        ("Shipping", "Standard and express delivery."),
        ("Returns", "Within 30 days.")
    ]
    .value(selected)
    .on_value_change(changed)
}
```

Use record keys for a collection whose order can change. This example keeps the application's `u64` identifiers through the selection handler.

```rust
use icedcn::Slot;
use icedcn::widget::accordion;

fn records<'a, M: 'a>(
    items: impl IntoIterator<Item = (u64, String, String)>,
    selected: Vec<u64>,
    changed: impl Fn(Vec<u64>) -> M + 'a,
) -> Slot<'a, M> {
    accordion::keyed(
        items
            .into_iter()
            .map(|(id, title, body)| (id, (title, body))),
    )
    .multiple()
    .value(selected)
    .on_value_change(changed)
    .into()
}
```
