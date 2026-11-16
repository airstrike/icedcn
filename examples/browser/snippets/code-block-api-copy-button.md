### Copy Button

A ghost icon button that copies its value to the clipboard.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::copy_button(value)` | `impl Into<Cow<str>>` | - | Required. The text to copy. |
| `.disabled(bool)` | `bool` | `false` | Disable copying, such as while a stream is incomplete. |
