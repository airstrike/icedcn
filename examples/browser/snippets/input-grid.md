```rust
use iced::Fill;
use icedcn::widget::{field, input, row};

row![
    field(input("Jordan").width(Fill)).label("First Name").width(Fill),
    field(input("Lee").width(Fill)).label("Last Name").width(Fill)
]
.spacing(20)
.width(384)
```
