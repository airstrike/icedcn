```rust
use iced::alignment::Horizontal;
use icedcn::widget::{bubble, code_block, column, message, text};

column![
    message(bubble("How do I filter notebook titles?").style(bubble::Muted))
        .align(Horizontal::Right),
    message(
        column![
            text::p("Compare the query and each title in lowercase."),
            code_block(code_block::content(CODE).language("rust")).header(
                code_block::header("src/search.rs").actions(code_block::copy_button(CODE))
            ),
            text::p("The result borrows the matching titles from the original list."),
        ]
        .spacing(12)
    ),
]
.spacing(16)

const CODE: &str = r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##;
```
