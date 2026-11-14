```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(DESTRUCTIVE)
        .font(Font::new("Geist Mono"))
        .wrap(true)
        .style(code_block::Destructive)
        .title("Error")
        .actions(code_block::copy_button(DESTRUCTIVE))
}

const DESTRUCTIVE: &str = r##"thread 'main' panicked at src/notebook.rs:18:22:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` to display a backtrace"##;
```
