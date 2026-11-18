```rust
use icedcn::widget::{code_block, column, tabs};

struct File {
    id: &'static str,
    title: &'static str,
    language: &'static str,
    code: &'static str,
}

tabs(|t| {
    let mut content = column![
        t.list(
            FILES
                .iter()
                .map(|file| t.trigger(file.id, file.title.rsplit('/').next().unwrap())),
        )
    ]
    .spacing(8);
    for file in FILES {
        content = content.push(
            t.content(
                file.id,
                code_block(
                    code_block::content(file.code)
                        .language(file.language)
                        .line_numbers(true),
                )
                .header(
                    code_block::header(file.title).actions(code_block::copy_button(file.code)),
                ),
            ),
        );
    }
    content.into()
})
.default_value("view")

const FILES: &[File] = &[
    File {
        id: "view",
        title: "src/view.rs",
        language: "rust",
        code: r##"use icedcn::widget::{column, text};
use icedcn::Slot;

pub fn view(titles: &[String]) -> Slot<'_, ()> {
    let mut items = column![].spacing(8);
    for title in titles {
        items = items.push(text::p(title.as_str()).into());
    }
    items.into()
}"##,
    },
    File {
        id: "search",
        title: "src/search.rs",
        language: "rust",
        code: r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##,
    },
    File {
        id: "config",
        title: ".cargo/config.toml",
        language: "toml",
        code: r##"[env]
RUST_LOG = "notebook=debug""##,
    },
];
```
