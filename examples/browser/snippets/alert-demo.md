```rust
use icedxdy::widget::{alert, column};
use crate::icon;

column![
    alert()
        .icon(icon::circle_check_big())
        .title("Payment successful")
        .description(
            "Your payment of $29.99 has been processed. A receipt has been sent to your email address.",
        )
        .max_width(448),
    alert()
        .icon(icon::info())
        .title("New feature available")
        .description(
            "We've added dark mode support. You can enable it in your account settings.",
        )
        .max_width(448),
]
.spacing(16)
```
