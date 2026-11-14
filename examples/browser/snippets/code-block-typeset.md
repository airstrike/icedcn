```rust
use iced::Widget;
use iced::widget::svg;
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
                .title(row![file_code(), code_block::title("src/index.rs")].spacing(6))
                .actions(code_block::copy_button(TYPESET)),
            markdown::view(&self.after, settings).map(|_| ()).boxed()
        ]
        .spacing(12)
        .into()
    }
}
fn file_code() -> Slot<'static, ()> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM14 2v6h6m-10 5-2 2 2 2m4-4 2 2-2 2"/></svg>"#)).width(14).height(14).style(|theme: &Theme,_|svg::Style{color:Some(theme.neutral.foreground.into())}).boxed().into()
}

const TYPESET: &str = r##"use std::path::Path;

fn load_titles(path: &Path) -> std::io::Result<Vec<String>> {
    let source = std::fs::read_to_string(path)?;
    Ok(source.lines().map(str::to_owned).collect())
}"##;
```
