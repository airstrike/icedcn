```rust
use crate::icon;
use icedcn::widget::trace::Status;
use icedcn::widget::{spinner, trace};

trace([
    line("Ran cargo fetch", icon::circle_check).into(),
    line("Failed to run cargo clippy", icon::circle_alert)
        .status(Status::Error)
        .into(),
    trace::line("Running cargo build")
        .icon(spinner().size(16.))
        .status(Status::Running)
        .into(),
    line("Run cargo test", icon::circle)
        .status(Status::Pending)
        .into(),
])
.into()
```
