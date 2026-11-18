```rust
use icedcn::widget::{column, item};

#[derive(Debug, Clone)]
enum Message {
    Open,
    OpenUrl(&'static str),
}

column![
    item::item([
        item::content([
            item::title("Visit our documentation").into(),
            item::description("Learn how to get started with our components.").into()
        ])
        .into(),
        item::actions([icon::chevron_right().size(16).into()])
    ])
    .on_press(Message::Open),
    item::item([
        item::content([
            item::title("External resource").into(),
            item::description("Opens the project in your browser.").into()
        ])
        .into(),
        item::actions([icon::external_link().size(16).into()])
    ])
    .style(item::Outline)
    .on_press(Message::OpenUrl("https://github.com/airstrike/icedcn"))
]
.spacing(16)
.width(Fill)
.max_width(448.0)
```
