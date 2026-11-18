```rust
use crate::icon;
use icedcn::widget::{button, code_block};

#[derive(Debug, Clone)]
enum Message {
    Wrap,
    Download,
}

code_block(
    code_block::content(CODE)
        .language("rust")
        .line_numbers(true)
        .wrap(wrap)
        .line_style(6..=10, code_block::Highlighted),
)
.header(
    code_block::header(code_block::title("src/view.rs").icon(icon::file_code())).actions(
        code_block::actions![
            button(icon::wrap_text())
                .style(button::Ghost)
                .square()
                .pressed(wrap)
                .on_press(Message::Wrap),
            button(icon::download())
                .style(button::Ghost)
                .square()
                .on_press(Message::Download),
            code_block::copy_button(CODE),
        ],
    ),
)

const CODE: &str = r##"use icedcn::{Slot, widget::{column, input, text}};

#[derive(Clone, Debug)]
pub enum Message { SearchChanged(String) }

pub fn view<'a>(query: &'a str, titles: &[&'a str]) -> Slot<'a, Message> {
    let search = input("Find a notebook").value(query.to_owned()).on_input(Message::SearchChanged);
    let mut results = column![search].spacing(8);
    for title in titles.iter().copied().filter(|title| title.contains(query)) {
        results = results.push(text::p(title).into());
    }
    results.into()
}"##;
```
