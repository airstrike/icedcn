### Title

A filename or a label, with an optional icon. It truncates.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::title(item)` | `impl Into<TitleItem>` | - | A label, or an icon followed by `.push(label)`. |
| `code_block::title![icon, label]` | - | - | A bare identifier first is `icon::name()` at 14px. |
