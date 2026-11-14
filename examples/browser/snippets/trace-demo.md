```rust
use icedcn::widget::{code_block, text, trace};

trace([
    trace::disclosure(|| trace::line("Thought for 5s"), || text(reason).into()).into(),
    trace::disclosure(
        || trace::line("Ran cargo --version"),
        || code_block("cargo 1.89.0").into(),
    )
    .into(),
])
```
