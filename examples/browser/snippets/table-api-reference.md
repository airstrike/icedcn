## API Reference

### Table

`table(children)` stacks the grid and caption in a horizontal scrollport.

| Function | Description |
| --- | --- |
| `table::header(rows)` | Rows with a bottom border. |
| `table::body(rows)` | Rows separated by borders. |
| `table::footer(rows)` | Medium-weight rows on a muted surface below a border. |
| `table::row(cells)` | `.border(false)` drops its bottom border. |
| `table::head(content)` | A 40px medium header cell with 8px horizontal padding. |
| `table::cell(content)` | A 14px cell with 8px padding. |
| `table::caption(content)` | Muted text below the table. |

A shared grid lays out all row groups against the same tracks. Set `Track::Fixed(px)` for a known width, `Track::Auto` to measure the widest cell in the column, or `Track::Flex(weight)` to share the remaining width. Attach groups using `.header(..)`, `.body(..)` and `.footer(..)`. A cell with `.span(n)` occupies adjacent tracks. Use `.selected(true)` to tint a selected row.

When the table overflows, focus its scrollport and use Left, Right, Home or End to reach the clipped columns.

A width on a single cell in an automatic column (for example `table::head("Invoice").width(100)`) is preferred when space permits. Its content determines the minimum scrollable width.
