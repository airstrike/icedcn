## API Reference

### `dialog(trigger, content)`

An iced `Component` that owns its open state. Both closures receive a `dialog::Handle` and build in your own message type: `trigger` what sits in place, `content` the panel shown above a backdrop while open. The panel is built only while open, so its widgets mount fresh each time.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled dialog starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the dialog: open exactly when the app says so. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state when the dialog opens or is dismissed. |
| `.show_close_button(bool)` | `bool` | `true` | Show the corner close button. |
| `.max_width(f32)` | `f32` | `384.0` | The panel's maximum width. |

### `dialog::Handle`

| Method | Description |
| --- | --- |
| `d.trigger(child)` | Pressing `child` opens the dialog. |
| `d.close(child)` | Pressing `child` closes it. The child also publishes its own `.on_press` message if configured. |
| `d.is_open()` | Whether the dialog is open. |

A press outside the panel and `Escape` dismiss it too. `dialog::header(title, description)` and `dialog::footer(actions)` lay out the header and footer.
