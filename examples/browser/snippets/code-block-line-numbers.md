```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message>() -> CodeBlock<'a, Message> {
    code_block(LINE_NUMBERS).font("Geist Mono").start_line(1)
}

const LINE_NUMBERS: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedcn = { path = "../icedcn" }"##;
```
