## API Reference

### `table::table()`

`table::table(children)` stacks its header, body, footer and caption.

| Function | Description |
| --- | --- |
| `table::header(rows)` | `TableHeader`: rows with a bottom border. |
| `table::body(rows)` | `TableBody`: rows separated by borders. |
| `table::footer(rows)` | `TableFooter`: `bg-muted/50` rows below a border. |
| `table::row(cells)` | `TableRow`. `.border(false)` drops its bottom border. |
| `table::head(content)` | `TableHead`: a 40px medium header cell. |
| `table::cell(content)` | `TableCell`: a `p-2` cell. |
| `table::caption(content)` | `TableCaption`: muted text below the table. |

A shared grid lays out all row groups against the same tracks. Set `Track::Fixed(px)` for a known width, `Track::Auto` to measure the widest cell in the column, or `Track::Flex(weight)` to share the remaining width. Attach groups using `.header(..)`, `.body(..)` and `.footer(..)`. A cell with `.span(n)` occupies adjacent tracks. Use `.selected(true)` to tint a selected row.
