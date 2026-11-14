```rust
use iced::Fill;
use icedxdy::widget::{field, input};

field(input("sk-...").secure(true).width(Fill))
    .label("API Key")
    .description("Your API key is encrypted and stored securely.")
    .width(320)
```
