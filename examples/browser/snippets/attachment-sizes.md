```rust
use icedxdy::Size;
use icedxdy::widget::{attachment, column};

column![
    attachment("Default attachment")
        .media(icon::file_text())
        .description("PDF · 2.4 MB"),
    attachment("Small attachment")
        .media(icon::file_text())
        .description("PDF · 2.4 MB")
        .size(Size::Sm),
    attachment("Extra small attachment")
        .media(icon::file_text())
        .size(Size::Xs),
].spacing(12)
```
