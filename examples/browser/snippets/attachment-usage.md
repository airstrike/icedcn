## Usage

```rust
use icedxdy::widget::{attachment, tooltip};

attachment("sales-dashboard.pdf")
    .media(icon::file_text())
    .description("PDF · 2.4 MB")
    .action(tooltip(
        attachment::action(icon::x()).on_press(Message::Remove("sales-dashboard.pdf")),
        "Remove sales-dashboard.pdf",
    ))
```
