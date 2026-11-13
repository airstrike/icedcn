```rust
field::group([
    field::set(text::label("Responses"), Some("Get notified when ChatGPT responds to requests that take time, like research or image generation."),
        field(checkbox().default_checked(true).disabled(true)).orientation(Horizontal).label(text::small("Push notifications"))).into(),
    separator().into(),
    field::set(text::label("Tasks"), Some("Get notified when tasks you've created have updates. Manage tasks"),
        field::group(["Push notifications", "Email notifications"].map(|label|
            field(checkbox()).orientation(Horizontal).label(text::small(label)).into()
        )).spacing(12)).into(),
]).width(320)
```
