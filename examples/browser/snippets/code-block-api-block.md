### CodeBlock

The surface that holds the header, content and footer.

Keyboard focus in the content shows a ring around the panel and changes the default border to the ring color. `.style(...)` replaces the default panel paint, so destructive and custom borders keep their color while the keyboard ring remains visible.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `code_block(source)` | `impl Into<Cow<str>>` | required | Plain selectable output. |
| `.style(style)` | `impl Into<StyleFn>` | theme default | Use `code_block::Destructive` for failed output. |
| `.compact(bool)` | `bool` | `false` | 12px code and 10px padding instead of 14px and 12px. |
| `.font(font)` | `Font` | `Font::MONOSPACE` | Code and gutter font. Explicit line children may override it. |
