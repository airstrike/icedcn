```rust
use icedcn::widget::{code_block, trace};

trace::disclosure(
    || trace::line("Failed to run cargo --version").status(trace::Status::Error),
    || code_block("command not found: cargo").into(),
)
```
