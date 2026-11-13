```rust
use icedxdy::widget::{trace, text};

trace([
    line("Ran pnpm install", icon::circle_check).into(),
    line("Failed to run pnpm lint", icon::circle_alert)
        .status(Status::Error)
        .into(),
    trace::line("Running pnpm build")
        .icon(spinner().size(16.))
        .status(Status::Running)
        .into(),
    line("Run pnpm test", icon::circle)
        .status(Status::Pending)
        .into(),
])
.into()
```
