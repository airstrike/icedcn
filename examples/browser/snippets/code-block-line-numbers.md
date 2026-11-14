```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(LINE_NUMBERS)
        .font(Font::new("Geist Mono"))
        .start_line(1)
}

const LINE_NUMBERS: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedxdy = { path = "../icedxdy" }"##;
```
