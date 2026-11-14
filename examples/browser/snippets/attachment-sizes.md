```rust
use icedcn::widget::container;
use icedcn::Size;
use icedcn::widget::{attachment, column};

container(column![
    attachment("Default attachment")
        .media(icon::file_text().size(16).selectable(false))
        .description("PDF · 2.4 MB"),
    attachment("Small attachment")
        .media(icon::file_text().size(16).selectable(false))
        .description("PDF · 2.4 MB")
        .size(Size::Sm),
    attachment("Extra small attachment")
        .media(icon::file_text().size(14).selectable(false))
        .size(Size::Xs),
].spacing(12).width(iced::Fill))
    .width(iced::Fill).max_width(384).padding([48, 0])
```
