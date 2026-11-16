```rust
use crate::icon;
use icedcn::widget::code_block;

code_block![
    code_block::header![code_block::title![file_code, "src/greet.rs"]],
    code_block::content("")
        .default_value(CODE)
        .language("rust")
        .line_numbers(true),
]

const CODE: &str = r##"fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}"##;
```
