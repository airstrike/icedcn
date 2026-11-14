## API Reference

### `select(entries)`

An iced `Component` that owns its open state and, uncontrolled, its value. The trigger shows the chosen label or the placeholder. Pressing it opens a modal list aligned with the selected option. The list scrolls when taller than the window.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.position(position)` | `select::Position` | `ItemAligned` | Align the selected row with the trigger, or use `Popper` for an edge-anchored list. |
| `.placeholder(text)` | `String` | `""` | Shown while nothing is chosen. |
| `.default_value(value)` | `String` | `None` | The value an uncontrolled select starts with. |
| `.value(value)` | `String` | uncontrolled | Control the select. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Publish a message with the chosen value. |
| `.disabled(bool)` | `bool` | `false` | Dims the trigger; it does not open. |
| `.width(length)` | `Length` | `Shrink` | The trigger's width. |
| `.size(size)` | `Size` | `Default` | `Size::Sm` is 28px tall. |
| `.style(select::Invalid)` | variant | `Default` | A destructive border and three-pixel ring. |
| `.min_width(pixels)` | `f32` | `144.0` | Minimum list width, constrained by the viewport. |
| `.side(side)`, `.align(align)` | `Side`, `Align` | `Bottom`, `Center` | Place a Popper list relative to its trigger. |
| `.icon(f)` | `Fn() -> Text` | built-in chevron | Replace the 16px trigger icon using an icon font. |

With the trigger focused, type the beginning of a label to select it. Enter, Space or an arrow key opens the list. Within the list, use the arrows, Home, End or a label prefix to focus an option. Enter selects it and Escape dismisses the list.

Use `.format_value(f)` to change how the trigger displays a selection and `.format_item(f)` to customize option content. For custom trigger appearance, use `.padding(..)`, `.font(..)`, `.icon_content(f)` or `.trigger_style(f)`.

### Entries

`select::item(value, label)` (with `.disabled(bool)`), `select::label(text)`, `select::separator()` and `select::group(entries)`.
