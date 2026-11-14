```rust
use icedcn::widget::tabs;

tabs(|t| {
    t.list([
        t.trigger("overview", "Overview"),
        t.trigger("analytics", "Analytics"),
        t.trigger("reports", "Reports"),
    ])
})
.line(true)
.default_value("overview")
```
