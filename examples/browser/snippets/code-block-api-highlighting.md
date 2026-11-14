### useHighlightedCode

The upstream hook corresponds to application-side parsing plus `CodeBlock::lines` in Rust. The fixed example helper reads retained Shiki token tables, renders one rich-text slot per line, and falls back to plain spans for unknown input. It is an example helper, not a public general-purpose parser.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `source` | `&str` | required | Code to display; highlighted recipes use fixed owned fixture data. |
| Language | fixed fixture metadata | retained with source | Uses the pinned grammar output for the fixture, otherwise plain output. |
| `visible` | byte count | full length | A UTF-8-safe prefix for streamed fixed samples. |
| `dark` | `bool` | active theme | GitHub Light or GitHub Dark token colors. |
