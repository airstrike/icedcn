## API Reference

### `dialog(trigger, content)`

Both closures receive a `dialog::Handle` and use your application’s message type. The first builds the trigger. The second builds the panel, which mounts fresh each time the dialog opens.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled dialog starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the dialog: open exactly when the app says so. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state when the dialog opens or is dismissed. |
| `.show_close_button(bool)` | `bool` | `true` | Show the corner close button. |
| `.max_width(f32)` | `f32` | `384.0` | The panel's maximum width. |
| `.scrollable_body(f)` | content closure | omitted | Place a scrollable body between the header and footer, up to half the window height. |
| `.footer(f)` | actions closure | omitted | Pin actions below the panel content. Below 640 pixels the actions stack in reverse visual order. |
| `.footer_align_x(alignment)` | `Horizontal` | `Right` | Align footer actions on wider windows. |
| `.padding(u16)` | pixels | `16` | Set the centered panel’s content padding. |
| `.margin(f32)` | pixels | `16.0` | Set minimum window-edge clearance. |
| `.backdrop(bool)` | `bool` | `true` | Dim the window behind the modal. |

### `dialog::Handle`

| Method | Description |
| --- | --- |
| `d.trigger(child)` | Pressing `child` opens the dialog. |
| `d.close(child)` | Pressing `child` closes it. The child also publishes its own `.on_press` message if configured. |
| `d.is_open()` | Whether the dialog is open. |

Press outside the panel or press Escape to dismiss it. Tab and Shift+Tab cycle through the panel’s controls. Focus returns to the trigger on dismissal. Use `dialog::header(title, description)` for the panel heading.
