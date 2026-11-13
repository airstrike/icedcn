```rust
use icedxdy::widget::{trace, text};

trace::disclosure(
    || trace::line("Looking into the slow deploy").status(Status::Running),
    move || {
        trace([
            trace::disclosure(
                || line("Searched for the checkout build log", icon::search),
                || {
                    row![
                        badge("vercel.com").style(badge::Secondary),
                        badge("github.com").style(badge::Secondary)
                    ]
                    .spacing(6)
                    .into()
                },
            )
            .into(),
            local("Read the last twelve deploys", "steps", index),
            result(
                "Searching for the cache key",
                icon::search,
                "cache key image optimization",
                Status::Running,
                false,
            ),
        ])
        .into()
    },
)
.default_open(true)
.into()
```
