```rust
use icedcn::widget::{text, trace};

trace::disclosure(
    || trace::line(if running { "Thinking" } else { "Thought" }),
    || text(reason).into(),
)
.default_open(running)
```
