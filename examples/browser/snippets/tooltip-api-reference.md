## API Reference

### `tooltip()`

`tooltip(trigger, content)` shows a small panel and arrow when the pointer rests on the trigger. The default delay is zero. The panel flips and shifts to stay in the window. Press Escape to hide it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `tooltip::Side` | `Top` | `Top`, `Right`, `Bottom` or `Left` of the trigger. |
| `.align(align)` | `tooltip::Align` | `Center` | `Start`, `Center` or `End` along the trigger. |
| `.side_offset(px)` | `f32` | `10.0` | The gap to the trigger, the arrow included. |
| `.delay(duration)` | `Duration` | `0` | `delayDuration`: how long the pointer rests before it shows. |
| `.open(bool)` | `bool` | uncontrolled | Display the tooltip while `open` is true. Handle its open-state messages to change this value. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state when hovering or dismissal requests a change. |

Keyboard focus does not open the tooltip.
