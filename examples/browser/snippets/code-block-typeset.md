```rust
use crate::icon;
use icedcn::widget::{code_block, column, markdown, text};

column![
    text::h3("Read a notebook index"),
    markdown::view(&before, settings),
    code_block![
        code_block::header![
            code_block::title![file_code, "src/index.rs"],
            code_block::actions![code_block::copy_button(CODE)],
        ],
        code_block::content(CODE).language("rust").line_numbers(true),
    ],
    markdown::view(&after, settings),
]
.spacing(12)

const CODE: &str = r##"use std::path::Path;

fn load_titles(path: &Path) -> std::io::Result<Vec<String>> {
    let source = std::fs::read_to_string(path)?;
    Ok(source.lines().map(str::to_owned).collect())
}"##;
```
