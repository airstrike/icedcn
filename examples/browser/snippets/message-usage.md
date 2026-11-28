## Usage

```rust
use iced::alignment::Horizontal;
use icedcn::widget::{bubble, message};

message(bubble("It is ready."))
    .align(Horizontal::Right)
    .footer("Delivered")
```
