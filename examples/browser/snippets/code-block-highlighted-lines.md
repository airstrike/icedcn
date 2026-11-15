```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message: 'static>() -> CodeBlock<'a, Message> {
    code_block(HIGHLIGHTED_LINES)
        .font("Geist Mono")
        .line_style(3, code_block::LineStyle::Highlighted)
        .line_style(4, code_block::LineStyle::Highlighted)
        .start_line(1)
}

const HIGHLIGHTED_LINES: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }"##;
```
