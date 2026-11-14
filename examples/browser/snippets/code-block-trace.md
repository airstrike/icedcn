```rust
use iced::widget::svg;
use iced::{Font, Widget};
use icedcn::{
    Slot, Theme,
    widget::{code_block, column, trace},
};
pub fn view() -> Slot<'static, ()> {
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
            trace::line(title).icon(terminal()).status(if failed {
                trace::Status::Error
            } else {
                trace::Status::Complete
            })
        },
        move || {
            let block = code_block(source)
                .font(Font::new("Geist Mono"))
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
fn terminal() -> Slot<'static, ()> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m4 17 6-5-6-5m8 12h8"/></svg>"#)).width(14).height(14).style(|theme: &Theme,_|svg::Style{color:Some(theme.neutral.foreground.into())}).boxed().into()
}

const TRACE: &str = r##"$ cargo --version
cargo 1.85.0 (d73d2caf9 2024-12-31)"##;

const TRACE_ERROR: &str = r##"$ cargo run --bin notes
error: no bin target named `notes` in default-run packages
help: available bin targets:
    notebook"##;
```
