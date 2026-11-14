### Syntax tokens

The example’s `listing` helper reads the token ranges for a fixed source and passes one rich-text slot per line to `CodeBlock::lines`. Unknown input displays in the default text color.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `source` | `&str` | required | Code to display; highlighted recipes use fixed owned fixture data. |
| Language | fixed fixture metadata | retained with source | Uses the pinned grammar output for the fixture, otherwise plain output. |
| `visible` | byte count | full length | A UTF-8-safe prefix for streamed fixed samples. |
| `dark` | `bool` | active theme | GitHub Light or GitHub Dark token colors. |
