```rust
use iced::Fill;
use icedcn::widget::{code_block, column};

let samples = [
    ("toml", "Cargo.toml", "[package]\nname = \"notebook\"\nversion = \"0.1.0\"\nedition = \"2024\""),
    ("rust", "src/main.rs", "fn main() {\n    println!(\"Notebook is ready.\");\n}"),
    ("text", "notes.txt", "A language that is not in the list renders as plain text."),
];

column(samples.map(|(language, title, code)| {
    code_block![
        code_block::header![title],
        code_block::content(code).language(language),
    ]
    .into()
}))
.width(Fill)
.spacing(16)
```
