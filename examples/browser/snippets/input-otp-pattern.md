```rust
use icedcn::widget::input_otp::Pattern;
use icedcn::widget::{field, input_otp};

field(input_otp(6).pattern(Pattern::Digits)).label("Digits Only")
```
