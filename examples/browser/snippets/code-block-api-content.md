### Content

The content that scrolls or wraps.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::content(body)` | `impl Into<Body>` | - | A string, or a list of `code_block::line`s. |
| `.language(name)` | `impl Into<Cow<str>>` | - | A language or alias such as `"rust"`, or `"ansi"` for terminal colors. |
| `.line_style(lines, style)` | `impl Into<LineRange>`, `LineStyle` | - | The state of one-based lines, such as `6..=10`. |
| `.wrap(bool)` | `bool` | `false` | Wrap long lines instead of scrolling. |
| `.line_numbers(bool)` | `bool` | `false` | Number each line. |
| `.start_line(n)` | `usize` | `1` | The number of the first line, when line numbers are on. |
| `.max_height(px)` | `impl Into<Pixels>` | - | Cap the height, such as `192`. |
| `.height(px)` | `impl Into<Pixels>` | - | An exact height. |
| `.focusable(bool)` | `bool` | `true` | Makes the content focusable so it can be scrolled by keyboard. |
| `.id(id)` | `impl Into<Id>` | - | Identifies the content for focus operations. |
| `.font(font)` | `impl Into<Font>` | `Font::MONOSPACE` | The code and line number font. |
| `.on_input(f)` | `impl Fn(String) -> Message` | - | Edit the source in place; each edit publishes the new source. |
| `.default_value(source)` | `impl Into<String>` | - | Edit in place from `source`, with the text kept in the block. |
