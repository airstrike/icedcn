```rust
use iced::{Center, Pixels};
use icedcn::Vertical;
use icedcn::widget::{row, separator, text};

row![
    text::small("Blog").line_height(Pixels(20.0)),
    separator().orientation(Vertical),
    text::small("Docs").line_height(Pixels(20.0)),
    separator().orientation(Vertical),
    text::small("Source").line_height(Pixels(20.0)),
]
.spacing(16)
.height(20)
.align_y(Center)
```
