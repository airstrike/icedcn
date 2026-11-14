## Usage

Pair each trigger with a panel using the same value. Set the initial tab with `.default_value(...)`.

```rust
use icedcn::widget::{column, tabs};

tabs(|t| {
    column![
        t.list([t.trigger("account", "Account"), t.trigger("password", "Password")]),
        t.content("account", "Make changes to your account here."),
        t.content("password", "Change your password here."),
    ]
    .into()
})
.default_value("account")
.width(400)
```
