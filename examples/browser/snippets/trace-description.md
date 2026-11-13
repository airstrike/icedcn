```rust
use icedxdy::widget::{trace, text};

trace([
    line(
        "Edited components/settings/notification-preferences-form.tsx",
        icon::file_pen,
    )
    .description("+48 −12")
    .into(),
    line("Ran pnpm test", icon::terminal)
        .description("14s")
        .into(),
    line("Searched for useChat", icon::search)
        .description("8 results")
        .into(),
])
.into()
```
