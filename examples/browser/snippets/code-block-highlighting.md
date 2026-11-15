```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message: 'static>() -> CodeBlock<'a, Message> {
    code_block(HIGHLIGHTING)
        .font("Geist Mono")
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
