```rust
use icedcn::widget::tabs;
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<'a, Message: Clone + 'static>(
    view_source: &'a str,
    search_source: &'a str,
    config_source: &'a str,
) -> Slot<'a, Message> {
    let files = [
        ("view", "view.rs", "src/view.rs", "rust", view_source),
        (
            "search",
            "search.rs",
            "src/search.rs",
            "rust",
            search_source,
        ),
        (
            "config",
            "config.toml",
            ".cargo/config.toml",
            "toml",
            config_source,
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
```
