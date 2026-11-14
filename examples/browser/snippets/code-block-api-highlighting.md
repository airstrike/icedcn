### useHighlightedCode

The upstream hook corresponds to application-side parsing plus `CodeBlock::lines` in Rust. The example helper caches iced token ranges, renders one rich-text slot per line, and falls back to plain spans for unknown input.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `source` | `&str` | required | Code to display; highlighted recipes use fixed owned fixture data. |
| `language` | parser token | required | A supported grammar or alias, otherwise plain output. |
| `visible` | byte count | full length | A UTF-8-safe prefix for streamed fixed samples. |
| `dark` | `bool` | active theme | GitHub Light or GitHub Dark token colors. |
