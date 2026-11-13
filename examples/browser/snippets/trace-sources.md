```rust
use icedxdy::widget::{trace, text};

trace::disclosure(
    || line("Used 3 sources", icon::globe),
    || {
        trace([
            source(
                "Install Astro | docs.astro.build",
                "https://docs.astro.build/en/install-and-setup/",
            ),
            source(
                "create-astro | npmjs.com",
                "https://www.npmjs.com/package/create-astro",
            ),
            source(
                "Settings: minimumReleaseAge | pnpm.io",
                "https://pnpm.io/settings#minimumreleaseage",
            ),
        ])
        .into()
    },
)
.default_open(true)
.into()
```
