```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(BASIC).font(Font::new("Geist Mono"))
}

const BASIC: &str = r##"$ cargo build --release
   Compiling icedxdy_core v0.1.0
   Compiling icedxdy_widget v0.1.0
   Compiling icedxdy v0.1.0
   Compiling notebook v0.1.0
    Finished `release` profile [optimized] target(s) in 4.20s"##;
```
