```rust
use iced::{Fill, Pixels, Widget};
use icedcn::widget::{column, rule, text};

column![
    text::h2("The People of the Kingdom")
        .size(30)
        .semibold()
        .line_height(Pixels(36.)),
    rule::horizontal(1).boxed(),
]
.spacing(8)
.width(Fill)
```
