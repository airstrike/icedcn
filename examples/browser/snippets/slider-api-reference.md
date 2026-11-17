## API Reference

### `slider()`

`slider(range)` creates an uncontrolled slider over `range`, starting at its start.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `T` | range start | The initial value of an uncontrolled slider. |
| `.value(value)` | `T` | - | Control the slider: it always shows `value`. |
| `.on_value_change(f)` | `Fn(T) -> Message` | `None` | The message published with the new value while dragging. |
| `.on_release(message)` | `Message` | `None` | Published when the thumb is released. |
| `.step(step)` | `impl AsPrimitive<f64>` | `1` | The step between values. |
| `.shift_step(step)` | `impl AsPrimitive<f64>` | `None` | The step while Shift is held. |
| `.disabled(disabled)` | `bool` | `false` | Ignore input and dim to half opacity. A disabled ancestor disables it too. |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | The track axis. A vertical track increases from bottom to top. |
| `.width(width)` | `impl Into<Length>` | `Fill`, or `16` when vertical | The slider's width. |
| `.height(height)` | `impl Into<Length>` | `16`, or `160` when vertical | The slider's height. |

Pass `slider::range(bounds)` for two thumbs or `slider::multiple(bounds, [a, b, c])` for an arbitrary fixed-size array. For either builder, `.value(...)` and `.default_value(...)` take an array of thumb values. The message from `.on_value_change(...)` carries the full array. Arrow keys step a focused thumb, Page Up and Page Down step by ten increments, and Home and End take it to its nearest allowed bound. A thumb stops when it reaches its neighbor.

Use `.orientation(Vertical)` for a vertical track, and `.height(...)` to set its length.
