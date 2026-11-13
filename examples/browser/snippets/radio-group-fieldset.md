```rust
pub(super) fn subscription_plan<'a>() -> Slot<'a, Message> {
    field::set(text::label("Subscription Plan"), Some("Yearly and lifetime plans offer significant savings."),
        radio_group(|g| field::group([
            ("monthly", "Monthly ($9.99/month)"), ("yearly", "Yearly ($99.99/year)"), ("lifetime", "Lifetime ($299.99)"),
        ].map(|(value,label)| field(g.item(value)).orientation(Horizontal).label(text::small(label)).into())).spacing(8).into())
        .default_value("monthly")
    ).max_width(320.).into()
}
```
