## API Reference

| Builder | Purpose |
| --- | --- |
| `date_picker()` | Open a Calendar inside an anchored Popover. |
| `.value(selection)` / `.default_value(selection)` | Control or seed the selected dates. |
| `.on_value_change(f)` | Publish a message with `Some(selection)` when dates are selected or `None` when the selection is cleared. |
| `.mode(Mode::Range)` | Edit the endpoints across two months. The popover stays open until dismissed. |
| `.caption_dropdown(true)` | Jump to another year or month. |
| `.placeholder(text)` / `.label(formatter)` | Change trigger content and date formatting. |
| `.disabled(true)` | Disable the picker. |
