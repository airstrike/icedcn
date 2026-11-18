```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::{code_block, markdown};

struct Reply {
    complete: bool,
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
        let language = language.unwrap_or("text");
        code_block(code_block::content(code).language(language))
            .header(
                code_block::header(language)
                    .actions(code_block::copy_button(code).disabled(!self.complete)),
            )
            .into()
    }
}

markdown::view_with(&items, settings, &reply)
```
