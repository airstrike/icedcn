```rust
use iced::Font;
use icedcn::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(HEADER)
        .font(Font::new("Geist Mono"))
        .title(code_block::terminal_title("Terminal"))
        .actions(code_block::copy_button("cargo check -p icedcn_widget"))
}

const HEADER: &str = r##"$ cargo check -p icedcn_widget
    Checking icedcn_core v0.1.0
    Checking icedcn_widget v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.80s"##;
```
