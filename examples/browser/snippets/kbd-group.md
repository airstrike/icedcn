```rust
use icedcn::widget::{kbd, row, text};

row![
    text::muted("Use"),
    kbd::group![kbd("Ctrl + B"), kbd("Ctrl + K")],
    text::muted("to open the command palette"),
]
.spacing(4)
.align_y(Center)
```
