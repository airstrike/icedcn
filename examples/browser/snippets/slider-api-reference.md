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
| `.width(width)` | `impl Into<Length>` | `Fill` | The slider's width. |

Pass `slider::range(bounds)` for two thumbs or `slider::multiple(bounds, [a, b, c])` for an arbitrary fixed-size array. Their value, default value and change callback all use the same array type. Arrow keys step a focused thumb, Page Up and Page Down step by ten increments, and Home and End take it to its nearest allowed bound. A thumb stops when it reaches its neighbor.

Use `.orientation(Vertical).height(160)` for a vertical track.
