### Bubble

The root bubble wrapper. `bubble(content)` accepts `impl Into<Slot<'a, Message>>`. Composition requires `Message: Clone` like Button. Passive content remains selectable.

| Builder | Type | Default | Behavior |
|---|---|---|---|
| `.style(variant)` | `bubble::Variant` | `Primary` | Primary, Secondary, Muted, Tinted, Outline, Ghost or Destructive; retains hover/focus behavior. |
| `.align(align)` | `Horizontal` | Inherited, otherwise `Left` | Align within the full conversation row. |
| `.interactive(enabled)` | `bool` | `false` | Make the whole surface focusable and pressable, even without a handler. |
| `.on_press(message)` | `Message` | None | Enable the surface and publish a message on pointer/keyboard activation. |
| `.disabled(disabled)` | `bool` | `false` | Disable input and descendants, including when inherited. |
| `.map_style(paint)` | `Fn(&Theme, bubble::Status, bubble::Style) -> bubble::Style` | None | Adjust resolved paint while retaining variant geometry and focus ring. |

`bubble::Status` is Active, Hovered, Pressed, Focused { is_hovered }, or Disabled. There is no hover animation for passive content. Interactive surfaces reuse Button's hover transition and keep stable geometry when pressed.
