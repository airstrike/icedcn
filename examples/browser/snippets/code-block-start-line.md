```rust
use icedcn::widget::code_block;

code_block![
    code_block::header!["src/notebooks.rs"],
    code_block::content(CODE)
        .language("rust")
        .line_numbers(true)
        .start_line(995),
]

const CODE: &str = r##"    let query = self.query.trim().to_lowercase();

    self.notebooks
        .iter()
        .filter(|book| book.title.to_lowercase().contains(&query))
        .map(|book| book.title.as_str())
        .collect()
}"##;
```
