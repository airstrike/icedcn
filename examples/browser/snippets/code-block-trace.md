```rust
use crate::icon;
use icedcn::{
    Slot,
    widget::{code_block, column, trace},
};
pub fn view<'a>() -> Slot<'a, ()> {
    column![
        transcript("Ran cargo --version", TRACE, false),
        transcript("Failed to run cargo run --bin notes", TRACE_ERROR, true)
    ]
    .spacing(8)
    .into()
}
fn transcript(title: &'static str, source: &'static str, failed: bool) -> Slot<'static, ()> {
    trace::disclosure(
        move || {
            trace::line(title)
                .icon(icon::terminal().size(14).line_height(1.0))
                .status(if failed {
                    trace::Status::Error
                } else {
                    trace::Status::Complete
                })
        },
        move || {
            let block = code_block(source)
                .font("Geist Mono")
                .compact(true)
                .wrap(true)
                .max_height(256.);
            if failed {
                block.style(code_block::Destructive).into()
            } else {
                block.into()
            }
        },
    )
    .default_open(true)
    .into()
}

const TRACE: &str = r##"$ cargo --version
cargo 1.85.0 (d73d2caf9 2024-12-31)"##;

const TRACE_ERROR: &str = r##"$ cargo run --bin notes
error: no bin target named `notes` in default-run packages
help: available bin targets:
    notebook"##;
```
