# Composition

```rust
use icedcn::widget::{button, column, drawer};

drawer(
    |d| d.trigger(button("Open")),
    |d| {
        column![
            drawer::header("Title", "Description"),
            drawer::footer([d.close(button("Done"))])
        ]
        .into()
    },
)
```
