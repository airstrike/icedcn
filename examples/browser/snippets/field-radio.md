```rust
use icedcn::Horizontal;
use icedcn::View;
use icedcn::widget::{field, radio_group, text};

fn plans<'a, M: Clone + 'static>() -> View<'a, M> {
    let plans = [
        ("monthly", "Monthly ($9.99/month)"),
        ("yearly", "Yearly ($99.99/year)"),
        ("lifetime", "Lifetime ($299.99)"),
    ];
    field::set(
        field::label("Subscription Plan"),
        Some("Yearly and lifetime plans offer significant savings."),
        radio_group(move |g| {
            field::group(plans.map(|(id, name)| {
                field(g.item(id))
                    .orientation(Horizontal)
                    .label(text::small(name))
                    .into()
            }))
            .spacing(8)
            .into()
        })
        .default_value("monthly"),
    )
    .max_width(320.)
    .into()
}
```
