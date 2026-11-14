```rust
use icedcn::theme;
use icedcn::widget::{field, input};

field(
    input("you@example.com")
        .width(Fill)
        .style(theme::text_input::Invalid),
)
.label("Email")
.error("Please enter a valid email address.")
```
