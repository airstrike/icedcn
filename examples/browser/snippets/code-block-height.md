```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message>(logs: &'a str) -> CodeBlock<'a, Message> {
    code_block(logs)
        .font("Geist Mono")
        .max_height(192.)
        .title("Logs")
        .actions(code_block::copy_button(logs))
}
```
