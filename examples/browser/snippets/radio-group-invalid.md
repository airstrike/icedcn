```rust
field::set(text::label("Notification Preferences"), Some("Choose how you want to receive notifications."),
    radio_group(|g| field::group([
        ("email", "Email only"), ("sms", "SMS only"), ("both", "Both Email & SMS"),
    ].map(|(value,label)| field(g.item(value).invalid(true)).orientation(Horizontal)
        .label(text::small(label).style(theme::text::Destructive)).into())).spacing(8).into())
    .default_value("email")).max_width(320.)
```
