```rust
use icedxdy::widget::{code_block, text, trace};

trace([
    trace::disclosure(|| trace::line("Thought for 5s"), || text(reason).into()).into(),
    trace::disclosure(
        || trace::line("Ran pnpm --version"),
        || code_block("10.18.0").into(),
    )
    .into(),
])
```
