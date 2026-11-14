### Styled lines

Supply `(LineStyle, Slot)` values to `CodeBlock::lines`. Children contain code without diff prefixes; the widget paints separate markers.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `CodeBlock::lines(lines)` | iterator of `(LineStyle, Slot)` | required | One item per line; preserve empty lines. |
| `LineStyle` | `Default`, `Highlighted`, `Added`, `Removed` | `Default` | Line tint and optional + or − decoration. |
