### Syntax highlighting

Enable the `icedcn/highlighter` Cargo feature to use `.language(...)` with source strings. The active theme selects light or dark token colors.

| Builder | Argument | Behavior |
| --- | --- | --- |
| `.language(language)` | Language name such as `"rust"` or `"toml"` | Highlight the source with the named grammar. Unknown languages display as plain text. |
| `.line_style(line, style)` | One-based source line and `LineStyle` | Apply a line highlight or change marker. Source line 1 stays the first line when the gutter starts at a different number. |
| `.diff()` | None | Read the leading space, `+`, or `-` on each patch-body line and paint its marker separately. |

With `.diff().copyable(true)`, the clipboard receives each line’s text without its leading patch marker.
