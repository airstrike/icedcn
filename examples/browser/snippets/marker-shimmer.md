```rust
use icedcn::widget::status_marker::{Separator, shimmer};
use icedcn::widget::{column, status_marker};

column![
    status_marker(shimmer("Thinking...")),
    status_marker(shimmer("Reading 4 files")).style(Separator),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
