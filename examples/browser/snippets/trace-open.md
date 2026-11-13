```rust
use icedxdy::widget::{code_block, text, trace};

trace::disclosure(
    || trace::line("Ran pnpm --version"),
    || code_block("$ pnpm --version\n10.33.4").into(),
)
.open(open)
.on_open_change(Message::Toggle)
```
