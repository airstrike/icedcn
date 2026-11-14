## API Reference

### `sheet(trigger, content)`

A `Dialog` pinned to a window edge: the same component, closures and `dialog::Handle` (`s.trigger`, `s.close`, `s.is_open()`). The panel spans the window along its side, using three quarters of the window width on the left or right, capped at 384px on wider windows, and pads its sections itself through `sheet::header` and `sheet::footer`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `sheet::Side` | `Right` | The edge: `Top`, `Right`, `Bottom`, `Left`. |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled sheet starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the sheet. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state. |
| `.show_close_button(bool)` | `bool` | `true` | Show the corner close button. |
