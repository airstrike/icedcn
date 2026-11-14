```rust
use iced::alignment::Horizontal;
use icedcn::widget::{bubble, message, text};
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<Message: Clone + 'static>() -> Slot<'static, Message> {
    column![
        message(bubble("How do I filter notebook titles?").style(bubble::Variant::Muted))
            .align(Horizontal::Right),
        message(
            column![
                text::p("Compare the query and each title in lowercase."),
                code_block(CHAT)
                    .language("rust")
                    .font("Geist Mono")
                    .title("src/search.rs")
                    .copyable(true),
                text::p("The result borrows the matching titles from the original list.")
            ]
            .spacing(12)
        )
    ]
    .spacing(16)
    .into()
}

const CHAT: &str = r##"fn matches(query: &str, title: &str) -> bool {
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
