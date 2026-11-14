### Title

`code_block::title(text)` supplies a medium-weight title that truncates within the available width. Use a plain string for a short header. For a terminal glyph and a truncating label together, pass `code_block::terminal_title("Terminal")` to `.title(...)`.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `code_block::title(text)` | `impl IntoFragment` | required | Truncating title text; inherits normal or compact size. |
