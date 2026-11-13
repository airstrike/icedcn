```rust
use icedxdy::widget::tabs;

tabs(|t| {
    t.list([
        t.trigger("account", "Account"),
        t.trigger("password", "Password"),
        t.trigger("notifications", "Notifications"),
    ])
})
.orientation(tabs::Orientation::Vertical)
.default_value("account")
```
