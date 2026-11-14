## API Reference

### `textarea()`

`textarea(placeholder)` is a multi-line field with the Input's look: at least 64px tall, growing with its text.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The initial text of an uncontrolled textarea. |
| `.value(value)` | `impl Into<String>` | - | Control the textarea: it always shows `value`. |
| `.on_input(f)` | `Fn(String) -> Message` | `None` | The message published with the complete new text on every edit. |
| `.disabled(disabled)` | `bool` | `false` | Ignore input and dim. A disabled ancestor disables it too. |
| `.min_height(height)` | `f32` | `64.0` | The least height in pixels. |
| `.style(style)` | `impl Into<text_editor::StyleFn>` | `textarea::Default` | `textarea::Invalid` for an invalid value. |

A focused text area uses the theme’s ring color for its border.
