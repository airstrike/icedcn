```rust
use icedcn::widget::drawer::Side;
use icedcn::widget::row;

row![
    panel("Top", Side::Top, 0.5),
    panel("Right", Side::Right, 0.5),
    panel("Bottom", Side::Bottom, 0.5),
    panel("Left", Side::Left, 0.5)
]
.spacing(8)
```
