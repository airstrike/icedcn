```rust
use icedcn::widget::code_block;

code_block(code_block::content(CODE).line_numbers(true))

const CODE: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }"##;
```
