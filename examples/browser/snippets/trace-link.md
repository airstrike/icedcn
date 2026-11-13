```rust
use icedxdy::widget::{trace, text};

trace([
    local("Read apps/v4/lib/docs.ts", "link", index),
    source(
        "Install Astro | docs.astro.build",
        "https://docs.astro.build/en/install-and-setup/",
    ),
])
.into()
```
