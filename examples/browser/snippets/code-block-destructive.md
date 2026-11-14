```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message>() -> CodeBlock<'a, Message> {
    code_block(DESTRUCTIVE)
        .font("Geist Mono")
        .wrap(true)
        .style(code_block::Destructive)
        .title("Error")
        .copyable(true)
}

const DESTRUCTIVE: &str = r##"thread 'main' panicked at src/notebook.rs:18:22:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` to display a backtrace"##;
```
