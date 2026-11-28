```rust
use iced::Center;
use icedcn::widget::{column, kbd, text};

column![
    kbd::group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")],
    kbd::group![kbd("Ctrl"), text::small("+"), kbd("B")],
]
.spacing(16)
.align_x(Center)
```
