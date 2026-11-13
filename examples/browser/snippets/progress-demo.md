```rust
use iced::Fill;
use icedxdy::widget::{container, progress};
use std::time::{Duration, Instant};

fn upload_progress<'a, Message: 'a>(started: Instant) -> icedxdy::Slot<'a, Message> {
    let value = if started.elapsed() >= Duration::from_millis(500) { 66.0 } else { 13.0 };
    container(progress(value).width(Fill))
        .max_width(450)
        .width(Fill)
        .into()
}
```
