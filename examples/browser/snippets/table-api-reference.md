## API Reference

### `table::table()`

`table::table(children)` stacks its header, body, footer and caption.

| Function | Description |
| --- | --- |
| `table::header(rows)` | Rows with a bottom border. |
| `table::body(rows)` | Rows separated by borders. |
| `table::footer(rows)` | Muted rows below a border. |
| `table::row(cells)` | `.border(false)` drops its bottom border. |
| `table::head(content)` | A 40px medium header cell. |
| `table::cell(content)` | A cell with 8px padding. |
| `table::caption(content)` | Muted text below the table. |

A shared grid lays out all row groups against the same tracks. Set `Track::Fixed(px)` for a known width, `Track::Auto` to measure the widest cell in the column, or `Track::Flex(weight)` to share the remaining width. Attach groups using `.header(..)`, `.body(..)` and `.footer(..)`. A cell with `.span(n)` occupies adjacent tracks. Use `.selected(true)` to tint a selected row.
