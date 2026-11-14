`.interactive(true)` makes the Bubble surface a button. A handler is optional, so a surrounding toast trigger can observe presses. `.on_press(message)` enables the surface and publishes the application's message. For link behavior, handle that message in `update` with your navigation task.

```rust
use icedxdy::widget::bubble;

bubble("Click here").style(bubble::Muted).interactive(true)
```
