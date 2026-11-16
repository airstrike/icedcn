```rust
use icedcn::widget::code_block;

code_block![
    code_block::header!["src/notebook.rs"],
    code_block::content(CODE).language("rust"),
]

const CODE: &str = r##"#[derive(Debug, Default)]
struct Notebook {
    title: String,
    pages: Vec<String>,
    archived: bool,
}"##;
```
