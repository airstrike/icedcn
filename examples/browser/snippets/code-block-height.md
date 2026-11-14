```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<'a, Message: Clone + 'a>(logs: &'a str) -> CodeBlock<'a, Message> {
    code_block(logs)
        .font(Font::new("Geist Mono"))
        .max_height(192.)
        .title("Logs")
        .actions(code_block::copy_button(logs))
}
```
