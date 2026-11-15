```rust
use icedcn::widget::{text, trace};

trace([
    line("Edited src/settings/notifications.rs", icon::file_pen)
        .description("+48 −12")
        .into(),
    line("Ran cargo test", icon::terminal)
        .description("14s")
        .into(),
    line("Searched for ChatState", icon::search)
        .description("8 results")
        .into(),
])
.into()
```
