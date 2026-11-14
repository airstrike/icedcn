```rust
use iced::Font;
use icedcn::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(HIGHLIGHTING)
        .font(Font::new("Geist Mono"))
        .language("rust")
        .title("src/notebook.rs")
}

const HIGHLIGHTING: &str = r##"#[derive(Debug, Default)]
struct Notebook {
    title: String,
    pages: Vec<String>,
    archived: bool,
}"##;
```
