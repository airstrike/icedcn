## API Reference

[Read the accordion implementation](https://github.com/airstrike/icedxdy/blob/HEAD/widget/src/accordion.rs).

### `accordion()`

Constructor for an `Accordion` from any `IntoIterator<Item = accordion::Item>`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.mode(mode)` | `accordion::Mode` | `Single { collapsible: true }` | Disclosure mode — single-open (with optional collapsible) or multiple-open. |
| `.default_value(ids)` | `IntoIterator<Item = impl Into<String>>` | `[]` | Items an uncontrolled accordion starts with open. The open set then lives in the widget tree. |
| `.value(ids)` | `IntoIterator<Item = impl Into<String>>` | uncontrolled | Control the accordion: the given items are always the open ones. |
| `.on_value_change(f)` | `Fn(Vec<String>) -> Message` | `None` | Publish a message with the complete set of open item IDs after a trigger changes it. |
| `.style(style)` | `impl Into<accordion::StyleFn>` | `theme::accordion::Default` | Variant unit structs (`Default`, `Card`) or a hand-rolled `Spec`. |
| `.padding(padding)` | `impl Into<Padding>` | `Padding::ZERO` | Space between the accordion and the edges of its container. |

### `accordion::item()`

Constructor for one disclosure item. Takes a stable `id` (matched against the open-set) and trigger label content.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.content(body)` | `impl Into<Slot>` | `None` | Body revealed when the item is open. Required for the item to render anything useful below the trigger. |
| `.chevron_closed(icon)` | `impl Into<Slot>` | down chevron | Closed-state indicator. |
| `.chevron_open(icon)` | `impl Into<Slot>` | up chevron | Open-state indicator. |
| `.inset(pixels)` | `f32` | `0.0` | Horizontal inset for the trigger and its content. The divider spans the item. |
| `.disabled(bool)` | `bool` | `false` | A disabled trigger ignores input and renders at 50% opacity. An already-open body remains usable. |

### `accordion::Mode`

| Variant | Description |
| --- | --- |
| `Mode::Single { collapsible: bool }` | At most one item open at a time. `collapsible = true` allows zero open. |
| `Mode::Multiple` | Any subset open. |

### State model

The widget tree holds the open items by default. Use `.default_value(...)` to choose which items start open. When the application needs the selection, store the IDs from `.on_value_change(...)` and pass them back to `.value(...)`. The callback receives the full set after the single- or multiple-open rule has been applied.

Use `.style(...)` with a `theme::accordion` variant or a custom `Spec` to change the appearance.
