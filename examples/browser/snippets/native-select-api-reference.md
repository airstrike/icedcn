## API Reference

### `native_select()`

`native_select(items)` shows the chosen item, or the first item while nothing else is chosen.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | first item | The value an uncontrolled select starts on. |
| `.value(value)` | `impl Into<String>` | - | Control the select: it always shows `value`. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | The message published with the newly chosen value. |
| `.disabled(disabled)` | `bool` | `false` | Disable the select. A disabled ancestor disables it too. |
| `.size(size)` | `impl Into<Size>` | `Default` | `Sm` (28px) or `Default` (32px). |
| `.style(style)` | `impl Into<pick_list::StyleFn>` | `native_select::Default` | `native_select::Invalid` for an invalid value. |

### `native_select::item(value, label)`

Use `native_select::group(label, options)` to insert labeled, nonselectable sections. Flat lists use iced’s pick list. Grouped lists use the same anchored list as Select.
