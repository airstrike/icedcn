```rust
use icedcn::widget::{trace, text};

trace([
    local("Read examples/browser/src/page.rs", "link", index),
    source(
        "Getting Started | rust-lang.org",
        "https://www.rust-lang.org/learn/get-started",
    ),
])
.into()
```
