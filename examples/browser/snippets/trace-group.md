```rust
use icedcn::widget::{text, trace};

trace([
    line("Searched for rust-version", icon::search).into(),
    line("Read Cargo.toml", icon::file_text).into(),
    line("Ran cargo fetch", icon::terminal).into(),
])
.into()
```
