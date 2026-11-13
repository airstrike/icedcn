```rust
radio_group(|g| field::group([
    ("plus", "Plus", "For individuals and small teams."),
    ("pro", "Pro", "For growing businesses."),
    ("enterprise", "Enterprise", "For large teams and enterprises."),
].map(|(value,label,description)| field(g.item(value)).orientation(Horizontal)
    .control_last().label(label).description(description).card().into())).spacing(8).width(384).into())
.default_value("plus")
```
