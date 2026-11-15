## Usage

```rust
use icedcn::widget::{attachment, tooltip};

attachment("sales-dashboard.pdf")
    .media(icon::file_text().size(16).selectable(false))
    .description("PDF · 2.4 MB")
    .action(tooltip(
        attachment::action(icon::x().size(12).selectable(false))
            .on_press(Message::Remove("sales-dashboard.pdf")),
        "Remove sales-dashboard.pdf",
    ))
```
