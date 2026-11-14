```rust
use crate::icon;
use icedcn::widget::{row, tabs};

tabs(|t| {
    t.list([
        t.trigger(
            "preview",
            row![icon::app_window().size(16), "Preview"].spacing(6),
        ),
        t.trigger("code", row![icon::code().size(16), "Code"].spacing(6)),
    ])
})
.default_value("preview")
```
