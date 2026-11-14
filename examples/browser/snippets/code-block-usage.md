## Usage

```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(HEADER)
        .font(Font::new("Geist Mono"))
        .title(code_block::terminal_title("Terminal"))
        .actions(code_block::copy_button("cargo check -p icedxdy_widget"))
}

const HEADER: &str = r##"$ cargo check -p icedxdy_widget
    Checking icedxdy_core v0.1.0
    Checking icedxdy_widget v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.80s"##;
```
