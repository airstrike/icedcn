```rust
use icedxdy::widget::{badge, row};

row![
    badge("Badge"),
    badge("Secondary").style(badge::Secondary),
    badge("Destructive").style(badge::Destructive),
    badge("Outline").style(badge::Outline),
]
.spacing(8)
```
