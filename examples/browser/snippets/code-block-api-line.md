### Line

One line of code.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::line(content)` | `impl Into<Slot>` | - | The line, without a diff marker. |
| `.style(style)` | `LineStyle` | `LineStyle::Default` | `code_block::Added`, `Removed` or `Highlighted`. |
