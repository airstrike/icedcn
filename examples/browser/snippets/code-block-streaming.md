```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message: 'static>(received: &'a str, complete: bool) -> CodeBlock<'a, Message> {
    code_block(received)
        .language("rust")
        .font("Geist Mono")
        .start_line(1)
        .title("src/matcher.rs")
        .actions(code_block::copy_button(received).disabled(!complete))
}
```
