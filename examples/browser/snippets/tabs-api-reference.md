## API Reference

### `tabs(body)`

An iced `Component` that owns the selected value. `body` receives a `tabs::Handle` and builds the list, triggers and panels in your own message type. Only the active panel is mounted, as in Radix.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The tab uncontrolled tabs start on. |
| `.value(value)` | `impl Into<String>` | uncontrolled | Control the tabs: the given value is always selected. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Publish a message with the tab value when a trigger selects another tab. |
| `.orientation(orientation)` | `tabs::Orientation` | `Horizontal` | Sets layout and arrow-key direction. |
| `.line(line)` | `bool` | `false` | Draws a line along the active tab. |
| `.width(width)` | `impl Into<Length>` | `Shrink` | Width of the tabs. |

### `tabs::Handle`

| Method | Description |
| --- | --- |
| `t.list(triggers)` | `TabsList`: the muted pill holding the triggers. |
| `t.trigger(value, label)` | `TabsTrigger` selecting `value`. Call `.disabled(true)` to skip it during keyboard navigation and pointer activation. |
| `t.content(value, child)` | `TabsContent`: shown only while `value` is selected. |
| `t.value()` | The selected value. |
