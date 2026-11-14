### CardHeader

The `CardHeader` component is used for a title, description, and optional action.

`card::header(title)` keeps its children as `Slot`s until Compose. Strings use the default title style; a `Text` or `Slot` preserves its explicit style. Title and description have a 4px gap, with the action aligned at the top-right.

| Builder | Type | Default |
|---|---|---|
| `.description(content)` | `impl IntoDescription` | absent |
| `.action(content)` | `impl Into<Slot>` | absent |
| `.inset(pixels)` | `f32` | 16 |

`card::header_simple(title)` is equivalent to `card::header(title)`.
