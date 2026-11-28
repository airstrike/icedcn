```rust
use icedcn::widget::status_marker::Separator;
use icedcn::widget::{column, status_marker};

column![
    status_marker("Today").style(Separator),
    status_marker("Worked for 42s").style(Separator),
    status_marker("Conversation compacted").style(Separator),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
