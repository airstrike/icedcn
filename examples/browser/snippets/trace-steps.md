```rust
use icedcn::widget::{trace, text};

trace::disclosure(
    || trace::line("Looking into the slow Cargo build").status(Status::Running),
    move || {
        trace([
            trace::disclosure(
                || line("Searched for Cargo build timings", icon::search),
                || {
                    row![
                        badge("rust-lang.org").style(badge::Secondary),
                        badge("github.com").style(badge::Secondary)
                    ]
                    .spacing(6)
                    .into()
                },
            )
            .into(),
            local("Read the last twelve build reports", "steps", index),
            result(
                "Checking the release profile",
                icon::search,
                "cargo build --release --timings",
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
