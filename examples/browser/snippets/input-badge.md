```rust
use iced::Fill;
use icedcn::widget::{Slot, badge, field, input, row, space};

field(input("https://api.example.com/webhook").width(Fill))
    .label(Slot::from(
        row![
            field::label("Webhook URL"),
            space::horizontal(),
            badge("Beta").style(badge::Secondary),
        ]
        .align_y(iced::Center)
        .width(Fill),
    ))
    .width(320)
```
