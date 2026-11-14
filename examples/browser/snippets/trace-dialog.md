```rust
use icedxdy::widget::{trace, text};

dialog(
    |d| {
        d.trigger(trace::action(
            line("Looked up Cargo build caching", icon::globe)
                .trailing(icon::chevron_right().size(16)),
            None,
        ))
    },
    |_| {
        column![
            dialog::header(
                "Looked up Cargo build caching",
                "Three results were used in the response."
            ),
            trace([
                source(
                    "Caching build outputs",
                    "https://doc.rust-lang.org/cargo/reference/build-cache.html"
                ),
                source(
                    "Build profiles",
                    "https://doc.rust-lang.org/cargo/reference/profiles.html"
                ),
                source(
                    "Build scripts",
                    "https://doc.rust-lang.org/cargo/reference/build-scripts.html"
                )
            ])
        ]
        .spacing(16)
        .into()
    },
)
.into()
```
