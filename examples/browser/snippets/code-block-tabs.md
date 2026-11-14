```rust
use icedcn::widget::tabs;
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<Message: Clone + 'static>() -> Slot<'static, Message> {
    let files = [
        ("view", "view.rs", "src/view.rs", "rust", TABS_0),
        ("search", "search.rs", "src/search.rs", "rust", TABS_1),
        (
            "config",
            "config.toml",
            ".cargo/config.toml",
            "toml",
            TABS_2,
        ),
    ];
    tabs(move |t| {
        let mut content =
            column![t.list(files.map(|(id, label, _, _, _)| t.trigger(id, label)))].spacing(8);
        for (id, _, title, language, source) in files {
            content = content.push(
                t.content(
                    id,
                    code_block(source)
                        .language(language)
                        .font("Geist Mono")
                        .start_line(1)
                        .title(title)
                        .copyable(true),
                ),
            );
        }
        content.into()
    })
    .default_value("view")
    .into()
}

const TABS_0: &str = r##"use icedcn::widget::{column, text};
use icedcn::Slot;

pub fn view(titles: &[String]) -> Slot<'_, ()> {
    let mut items = column![].spacing(8);
    for title in titles {
        items = items.push(text::p(title.as_str()).into());
    }
    items.into()
}"##;

const TABS_1: &str = r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##;

const TABS_2: &str = r##"[env]
RUST_LOG = "notebook=debug""##;
```
