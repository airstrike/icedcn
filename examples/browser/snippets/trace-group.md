```rust
use icedxdy::widget::{trace, text};

trace([
    line("Searched for minimumReleaseAge", icon::search).into(),
    line("Read pnpm-workspace.yaml", icon::file_text).into(),
    line("Ran pnpm install", icon::terminal).into(),
])
.into()
```
