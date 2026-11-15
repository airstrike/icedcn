```rust
use icedcn::widget::{text, trace};

trace::disclosure(
    || line("Used 3 sources", icon::globe),
    || {
        trace([
            source(
                "Getting Started | rust-lang.org",
                "https://www.rust-lang.org/learn/get-started",
            ),
            source(
                "Creating a package | The Cargo Book",
                "https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html",
            ),
            source(
                "Configuration | The Cargo Book",
                "https://doc.rust-lang.org/cargo/reference/config.html",
            ),
        ])
        .into()
    },
)
.default_open(true)
.into()
```
