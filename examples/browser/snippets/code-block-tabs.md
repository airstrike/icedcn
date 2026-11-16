```rust
use icedcn::widget::{code_block, column, tabs};

struct File {
    id: &'static str,
    title: &'static str,
    language: &'static str,
    code: &'static str,
}

tabs(|t| {
    let mut content = column![t.list(
        FILES.iter().map(|file| t.trigger(file.id, file.title.rsplit('/').next().unwrap())),
    )]
    .spacing(8);
    for file in FILES {
        content = content.push(t.content(
            file.id,
            code_block![
                code_block::header![
                    file.title,
                    code_block::actions![code_block::copy_button(file.code)],
                ],
                code_block::content(file.code)
                    .language(file.language)
                    .line_numbers(true),
            ],
        ));
    }
    content.into()
})
.default_value("view")
```
