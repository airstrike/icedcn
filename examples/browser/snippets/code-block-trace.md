```rust
use crate::icon;
use icedcn::widget::{code_block, trace};
use icedcn::{Destructive, Size};

trace([
    trace::disclosure(
        || trace::line("Ran cargo --version").icon(icon::terminal().size(14)),
        || {
            code_block(code_block::content(OUTPUT).wrap(true).max_height(256))
                .size(Size::Sm)
                .into()
        },
    )
    .default_open(true)
    .into(),
    trace::disclosure(
        || {
            trace::line("Failed to run cargo run --bin notes")
                .icon(icon::terminal().size(14))
                .status(trace::Status::Error)
        },
        || {
            code_block(code_block::content(ERROR).wrap(true).max_height(256))
                .size(Size::Sm)
                .style(Destructive)
                .into()
        },
    )
    .default_open(true)
    .into(),
])

const OUTPUT: &str = r##"$ cargo --version
cargo 1.85.0 (d73d2caf9 2024-12-31)"##;

const ERROR: &str = r##"$ cargo run --bin notes
error: no bin target named `notes` in default-run packages
help: available bin targets:
    notebook"##;
```
