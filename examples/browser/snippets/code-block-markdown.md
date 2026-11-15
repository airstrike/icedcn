```rust
use iced::Element;
use icedcn::{
    Theme,
    widget::{code_block, markdown},
};

pub struct Reply {
    pub complete: bool,
}

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme> for Reply {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_block(
        &'a self,
        _settings: markdown::Settings,
        language: Option<&'a str>,
        code: &'a str,
        _lines: &'a [markdown::Text],
    ) -> Element<'a, markdown::Uri, Theme> {
        code_block(code)
            .language(language.unwrap_or("text"))
            .font("Geist Mono")
            .title(language.unwrap_or("Code"))
            .actions(code_block::copy_button(code).disabled(!self.complete))
            .into()
    }
}

pub fn view<'a>(
    items: &'a [markdown::Item],
    reply: &'a Reply,
    theme: &Theme,
) -> Element<'a, markdown::Uri, Theme> {
    let settings =
        markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()));
    markdown::view_with(items, settings, reply)
}
```
