```rust
use icedcn::widget::tabs;

tabs(|t| {
    t.list([
        t.trigger("home", "Home"),
        t.trigger("settings", "Disabled").disabled(true),
    ])
})
.default_value("home")
```
