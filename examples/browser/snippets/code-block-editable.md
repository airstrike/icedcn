```rust
use crate::icon;
use icedcn::widget::code_block;

code_block(
    code_block::content("")
        .default_value(CODE)
        .language("rust")
        .line_numbers(true),
)
.header(code_block::header(
    code_block::title("src/greet.rs").icon(icon::file_code()),
))

const CODE: &str = r##"fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}"##;
```
