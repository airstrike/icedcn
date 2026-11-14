## API Reference

### `progress()`

Constructor for a `Progress` builder. Takes the current value (clamped to `0.0..=100.0`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.value(value)` | `impl Into<f32>` | constructor arg | Update the value at builder time (clamped to `0..=100`). |
| `.width(width)` | `impl Into<Length>` | `iced::Fill` | Total width of the bar. Cap with a numeric value or wrap in a `container().max_width(...)` for shadcn's `w-[60%]` / `max-w-sm` patterns. |
| `.style(style)` | `impl Into<progress_bar::StyleFn>` | `theme::progress_bar::Default` | Override the style. |
| `.on_idle(f)` | `Fn(f32) -> Message` | `None` | Publish a message with the target value when the animation settles. |

The bar's height (`h-1` = 4px in nova) is fixed by the wrapper; restyle via `.style(...)` if you need a thicker bar.

The progress bar draws its own indicator. Use the functions in `theme::progress_bar` to style it.
