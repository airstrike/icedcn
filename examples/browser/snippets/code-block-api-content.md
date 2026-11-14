### Content viewport

The constructor owns the content. It scrolls horizontally by default and can wrap or cap its vertical viewport.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.wrap(bool)` | `bool` | `false` | Wrap long lines instead of horizontal scrolling. |
| `.start_line(usize)` | `usize` | absent | Enable numbering from this value; gutter grows with the last number. |
| `.height(f32)` | pixels | absent | Exact viewport height. |
| `.max_height(f32)` | pixels | absent | Maximum viewport height, preserving short content height. |
| `.collapsed(bool)` | `bool` | `false` | Clip capped content and let vertical wheel input reach the document. |
| `.focusable(bool)` | `bool` | `true` | Include content in keyboard traversal. |
| `.id(id)` | `impl Into<Id>` | absent | ID of the focusable viewport. |
