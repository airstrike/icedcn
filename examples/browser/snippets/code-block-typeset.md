```rust
use crate::icon;
use iced::Widget;
use icedcn::{
    Slot, Theme,
    widget::{code_block, column, markdown, row, text},
};
pub fn view<'a>(
    before: &'a [markdown::Item],
    source: &'a str,
    after: &'a [markdown::Item],
    theme: &Theme,
) -> Slot<'a, ()> {
    let settings =
        markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()));
    column![
        text::h3("Read a notebook index"),
        markdown::view(before, settings).map(|_| ()).boxed(),
        code_block(source)
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
        markdown::view(after, settings).map(|_| ()).boxed()
    ]
    .spacing(12)
    .into()
}
```
