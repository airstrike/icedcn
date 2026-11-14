### Interactive Bubbles

When a bubble is clickable, use `.interactive(true)` or `.on_press(message)` so the whole surface is focusable and receives Enter and Space. Bubble supplies a visible keyboard focus ring; its visible text identifies the action. `.disabled(true)` also disables descendant controls and skips the surface in focus traversal.

```rust
use iced::alignment::Horizontal;
use icedxdy::Slot;
use icedxdy::widget::bubble;

#[derive(Clone)]
enum Message { Reply }

fn example() -> Slot<'static, Message> {
    bubble("I forgot my password")
        .style(bubble::Muted)
        .align(Horizontal::Right)
        .on_press(Message::Reply)
        .into()
}
```

Handle `Message::Reply` in the application's `update`. For icon-only actions, include adjacent explanatory text or a visible tooltip; those aids do not replace backend accessibility labels where supported.
