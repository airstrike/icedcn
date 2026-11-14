```rust
use icedxdy::widget::{container, text};
use icedxdy::theme::container::CardMuted;
use icedxdy::{radius, Size::Sm};

container(text::inline_code("icedxdy::widget::alert_dialog"))
    .padding([3, 5])
    .class(CardMuted + radius(Sm))
```
