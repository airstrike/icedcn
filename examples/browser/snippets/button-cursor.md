## Cursor

Tailwind v4 [switched](https://tailwindcss.com/docs/upgrade-guide#buttons-use-the-default-cursor) from `cursor: pointer` to `cursor: default` for the button component.

icedxdy also uses the default arrow. To keep the pointer behavior for an action, use `.interaction(Interaction::Pointer)`:

```rust
use iced::mouse::Interaction;
use icedxdy::widget::button;

button("Docs").style(button::Ghost)
    .interaction(Interaction::Pointer)
    .on_press(Message::OpenDocs)
```

Handle `Message::OpenDocs` in the application's `update`. Use `.anchor()` for a navigation action; it also enables the primary variant's anchor hover style.
