```rust
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<Message: 'static>() -> Slot<'static, Message> {
    let files = [
        ("Cargo.toml", "toml", LANGUAGES_0),
        ("src/main.rs", "rust", LANGUAGES_1),
        ("notes.txt", "text", LANGUAGES_2),
    ];
    column(files.map(|(title, language, source)| {
        code_block(source)
            .font("Geist Mono")
            .language(language)
            .title(title)
            .into()
    }))
    .spacing(16)
    .into()
}

const LANGUAGES_0: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024""##;

const LANGUAGES_1: &str = r##"fn main() {
    println!("Notebook is ready.");
}"##;

const LANGUAGES_2: &str = r##"Notebook shortcuts
Enter    Save the current note
Escape   Close the search field"##;
```
