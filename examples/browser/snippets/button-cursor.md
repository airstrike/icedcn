## Cursor

Buttons use the default arrow cursor. Set `.interaction(Interaction::Pointer)` to show a pointer while hovering.

```rust
use iced::mouse::Interaction;
use icedcn::widget::button;

button("Docs")
    .style(button::Ghost)
    .interaction(Interaction::Pointer)
    .on_press(Message::OpenDocs)
```

Handle `Message::OpenDocs` in the application's `update`. Use `.anchor()` for a navigation action; it also enables the primary variant's anchor hover style.
