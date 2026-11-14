## API Reference

### `tabs(body)`

An iced `Component` that owns the selected value. `body` receives a `tabs::Handle` and builds the list, triggers and panels in your own message type. Only the active panel is mounted.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The tab uncontrolled tabs start on. |
| `.value(value)` | `impl Into<String>` | uncontrolled | Control the tabs: the given value is always selected. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Publish a message with the tab value when a trigger selects another tab. |
| `.orientation(orientation)` | `tabs::Orientation` | `Horizontal` | Sets layout and arrow-key direction. |
| `.line(line)` | `bool` | `false` | Draws a line along the active tab. |
| `.list_width(width)` | `impl Into<Length>` | `Shrink` | Set the strip width. Horizontal triggers share extra width. |
| `.rounded()` | | | Round the strip and its triggers into pills. |
| `.width(width)` | `impl Into<Length>` | `Shrink` | Width of the tabs. |

### `tabs::Handle`

| Method | Description |
| --- | --- |
| `t.list(triggers)` | `TabsList`: the muted pill holding the triggers. |
| `t.trigger(value, label)` | `TabsTrigger` selecting `value`. Call `.disabled(true)` to skip it during keyboard navigation and pointer activation. |
| `t.content(value, child)` | `TabsContent`: shown only while `value` is selected. |
| `t.value()` | The selected value. |

Use Left/Right in a horizontal list or Up/Down in a vertical list. Home and End select the first and last enabled tab. Arrow navigation wraps, and Tab moves out of the list. When an intrinsic-width horizontal strip exceeds its viewport, keyboard navigation scrolls the focused trigger into view.
