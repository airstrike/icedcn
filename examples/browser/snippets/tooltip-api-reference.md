## API Reference

### `tooltip()`

`tooltip(trigger, content)` shows a small panel and arrow on hover or keyboard focus. The default delay is zero. The panel flips and shifts to stay in the window. Press Escape to hide it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `tooltip::Side` | `Top` | `Top`, `Right`, `Bottom` or `Left` of the trigger. |
| `.align(align)` | `tooltip::Align` | `Center` | `Start`, `Center` or `End` along the trigger. |
| `.side_offset(px)` | `f32` | `10.0` | The gap to the trigger, the arrow included. |
| `.delay(duration)` | `Duration` | `0` | `delayDuration`: how long the pointer rests before it shows. |
| `.padding(padding)` | `impl Into<Padding>` | `[6, 12]` | Space around the hint. |
| `.default_open(bool)` | `bool` | `false` | Initial visibility for an uncontrolled hint. |
| `.open(bool)` | `bool` | uncontrolled | Display the tooltip while `open` is true. Handle its open-state messages to change this value. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state when hovering or dismissal requests a change. |

Pressing the trigger dismisses its hint and runs the trigger’s action. Touch taps activate the trigger. A disabled button can show a pointer hint and is skipped during keyboard traversal.
