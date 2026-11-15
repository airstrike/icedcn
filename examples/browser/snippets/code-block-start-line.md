```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message: 'static>() -> CodeBlock<'a, Message> {
    code_block(START_LINE)
        .font("Geist Mono")
        .language("rust")
        .title("src/notebooks.rs")
        .start_line(995)
}

const START_LINE: &str = r##"    let query = self.query.trim().to_lowercase();

    self.notebooks
        .iter()
        .filter(|book| book.title.to_lowercase().contains(&query))
        .map(|book| book.title.as_str())
        .collect()
}"##;
```
