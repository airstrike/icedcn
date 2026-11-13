```rust
use icedxdy::widget::{code_block, text, trace};

trace::disclosure(
    || trace::line("Failed to run pnpm --version").status(trace::Status::Error),
    || code_block("command not found: pnpm").into(),
)
```
