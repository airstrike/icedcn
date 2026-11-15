```rust
use crate::icon;
use iced::Widget;
use icedcn::{
    Slot, Theme,
    widget::{code_block, column, markdown, row, text},
};
pub struct Article {
    before: Vec<markdown::Item>,
    after: Vec<markdown::Item>,
}
impl Default for Article {
    fn default() -> Self {
        Self {
            before: markdown::parse(
                "Read the notebook index with `read_to_string` and return one title per line.",
            ).collect(),
            after: markdown::parse(
                "The caller receives an `io::Result` and can display a read error in the application.",
            ).collect(),
        }
    }
}
impl Article {
    pub fn view(&self, theme: &Theme) -> Slot<'_, ()> {
        let settings =
            markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()));
        column![
            text::h3("Read a notebook index"),
            markdown::view(&self.before, settings).map(|_| ()).boxed(),
            code_block(TYPESET)
                .language("rust")
                .font("Geist Mono")
                .start_line(1)
                .title(
                    row![
                        icon::file_code().size(14).line_height(1.0),
                        code_block::title("src/index.rs")
                    ]
                    .spacing(6)
                )
                .copyable(true),
            markdown::view(&self.after, settings).map(|_| ()).boxed()
        ]
        .spacing(12)
        .into()
    }
}

const TYPESET: &str = r##"use std::path::Path;

fn load_titles(path: &Path) -> std::io::Result<Vec<String>> {
    let source = std::fs::read_to_string(path)?;
    Ok(source.lines().map(str::to_owned).collect())
}"##;
```
