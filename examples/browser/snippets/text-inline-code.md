```rust
use icedcn::widget::{container, text};
use icedcn::theme::container::CardMuted;
use icedcn::{radius, Size::Sm};

container(text::inline_code("widget::alert_dialog"))
    .padding([3, 5])
    .class(CardMuted + radius(Sm))
```
