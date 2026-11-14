```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(START_LINE)
        .font(Font::new("Geist Mono"))
        .language("rust")
        .title(code_block::title("src/notebooks.rs"))
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
