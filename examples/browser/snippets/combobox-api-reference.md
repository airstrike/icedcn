## API Reference

### `combobox(items)`

An iced `Component` that owns the typed text, the open state and, uncontrolled, the value. Focusing or typing in the input opens a list as wide as the input, showing the items that contain the text (ignoring case); choosing one fills the input and closes the list. `Escape` and a press outside close it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.placeholder(text)` | `String` | `""` | The input's placeholder. |
| `.empty(text)` | `String` | `"No items found."` | `ComboboxEmpty`, shown when nothing matches. |
| `.default_value(item)` | `String` | `None` | The item an uncontrolled combobox starts with. |
| `.value(item)` | `String` | uncontrolled | Control the combobox. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Receives the chosen item. |
| `.disabled(bool)` | `bool` | `false` | Disables the input. |
| `.width(length)` | `Length` | `256` | The input's width. |
| `.icon(f)` | `Fn() -> Text` | text glyph | The chevron at the input's end. |

Structured entries use `combobox::options`. Build each option with `combobox::item(value, label)`, then attach a `.group(label)` or `.description(text)`. `value` is the string delivered on selection.

| Builder | Behavior |
| --- | --- |
| `.multiple(true)` | Select several choices. Press a chip to remove it. |
| `.values(items)` / `.default_values(items)` | Set controlled or initial choices for multiple selection. |
| `.on_values_change(handler)` | Receive the complete updated selection. |
| `.auto_highlight(true)` | Highlight the first result after filtering. Up/Down move the highlight and Enter selects it. |
| `.clearable(true)` | Add a button to clear the current value. |
| `.invalid(true)` | Apply the invalid input style. |
| `.popup(true)` | Open a search field and result list from a separate trigger. |
| `.leading_icon(icon)` | Place the icon in the input's leading addon. |
