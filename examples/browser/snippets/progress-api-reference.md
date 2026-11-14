## API Reference

### `progress()`

Constructor for a `Progress` builder. Takes the current value (clamped to `0.0..=100.0`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.value(value)` | `impl Into<f32>` | constructor arg | Update the value at builder time (clamped to `0..=100`). |
| `.width(width)` | `impl Into<Length>` | `iced::Fill` | Total width of the bar. Cap with a numeric value or wrap in a `container().max_width(...)`. |
| `.style(style)` | `impl Into<progress_bar::StyleFn>` | `theme::progress_bar::Default` | Override the style. |
| `.on_idle(f)` | `Fn(f32) -> Message` | `None` | Publish a message with the target value when the animation settles. |

The progress indicator is 4px tall.

The progress bar draws its own indicator. Use the functions in `theme::progress_bar` to style it.
