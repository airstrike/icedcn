```rust
use icedxdy::widget::{code_block, text, trace};

trace::disclosure(
    || trace::line("Ran cargo --version"),
    || code_block("$ cargo --version\ncargo 1.89.0").into(),
)
.open(open)
.on_open_change(Message::Toggle)
```
