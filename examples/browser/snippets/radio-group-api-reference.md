## API Reference

### `radio_group()`

`radio_group(|g| ..)` owns the checked value; its closure receives a handle and lays out the items. A `field` around an item links its label: pressing the label checks the item.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The item an uncontrolled group starts on. |
| `.value(value)` | `impl Into<String>` | - | Control the group: `value` is always the checked item. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | The message published with the newly chosen value. |
| `.disabled(disabled)` | `bool` | `false` | Disable every item. A disabled ancestor disables them too. |

### `g.item(value)`

A 16px circle that fills with the primary color when checked. Use `.disabled(true)` to disable one item and `.invalid(true)` to show the validation ring.

Tab enters at the checked item, or the first enabled item when none is checked. The next Tab leaves the group. Arrow keys select the previous or next enabled item, wrapping at the ends. Home and End select the first or last enabled item.

### `radio(..)`

The standalone radio, `radio(label, value, selected, on_click)`, stays for app-owned values; it takes `.disabled(..)` too.
