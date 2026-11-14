```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(HIGHLIGHTED_LINES)
        .font(Font::new("Geist Mono"))
        .line_style(3, code_block::LineStyle::Highlighted)
        .line_style(4, code_block::LineStyle::Highlighted)
        .start_line(1)
}

const HIGHLIGHTED_LINES: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedxdy = { path = "../icedxdy" }"##;
```
