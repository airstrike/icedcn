## API Reference

### `item::item()`

`item::item(children)` lays its children out in a row with a 10px gap, 12px horizontal padding and 10px vertical padding inside a rounded box.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl Into<container::Spec>` | `item::Default` | `item::Outline`, `item::Muted`, or a tweaked `Spec`. |
| `.size(size)` | `item::Size` | `Default` | `Default`, `Sm` or `Xs`. |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | Lay the children out in a row or a column. |

### Slots

| Function | Description |
| --- | --- |
| `item::media(content)` | Unframed media; `.style(item::Icon)` or `.style(item::Image)` (a 40px clipped tile). |
| `item::content(children)` | A column that fills the available width. Use `.width(Shrink)` to size it to its contents. |
| `item::title(text)` | `ItemTitle` typography. |
| `item::description(text)` | `ItemDescription` typography. |
| `item::actions(children)` | A row of trailing actions. |
| `item::header(children)` / `item::footer(children)` | Full-width rows above and below. |
