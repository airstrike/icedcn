### CodeBlock

The surface that holds the header, content and footer.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `code_block(source)` | `impl Into<Cow<str>>` | required | Plain selectable output. |
| `.style(style)` | `impl Into<StyleFn>` | Nova default | Use `code_block::Destructive` for failed output. |
| `.compact(bool)` | `bool` | `false` | 12px code and 10px padding instead of 14px and 12px. |
| `.font(font)` | `Font` | `Font::MONOSPACE` | Code and gutter font. Explicit line children may override it. |
