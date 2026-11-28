```rust
use icedcn::theme;
use icedcn::widget::{field, radio_group, text};
use icedcn::{Horizontal, View};

fn notifications<'a, M: Clone + 'static>() -> View<'a, M> {
    let choices = radio_group(|g| {
        field::group(["Email only", "SMS only", "Both Email & SMS"].map(|name| {
            field(g.item(name).invalid(true))
                .orientation(Horizontal)
                .label(text::small(name).style(theme::text::Destructive))
                .into()
        }))
        .spacing(8)
        .into()
    })
    .default_value("Email only");
    field::set(
        field::label("Notification Preferences"),
        Some("Choose how you want to receive notifications."),
        choices,
    )
    .max_width(320.)
    .into()
}
```
