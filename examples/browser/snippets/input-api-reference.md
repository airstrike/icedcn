## API Reference

### `input()`

`input(placeholder)` starts empty and keeps the text the user enters. The placeholder and value can be owned or borrowed.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | Set the control height and text size. |
| `.text_size(px)` | `f32` | size-dependent | Set text size independently of control height. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Input width. Set `iced::Fill` to fill the parent. |
| `.default_value(value)` | `impl IntoFragment` | `""` | Initial text of an uncontrolled input. |
| `.value(value)` | `impl IntoFragment` | uncontrolled | Display text from application state. Handle edits in `update` to change the value. Without `.on_input(...)`, the controlled input is read-only. |
| `.on_input(f)` | `Fn(String) -> Message` | `None` | Publish a message with the full text after each edit, in either mode. |
| `.secure(bool)` | `bool` | `false` | Mask the displayed characters, as in a password field. The `.on_input` message carries the unmasked text. |
| `.disabled(bool)` | `bool` | `false` | Disable the input. A disabled ancestor (a disabled field) disables it too. |
| `.on_submit(message)` | `Message` | `None` | Publish `message` when the user presses Enter. |
| `.on_focus(message)` | `Message` | `None` | Publish `message` when the input gains focus. |
| `.on_blur(message)` | `Message` | `None` | Publish `message` when the input loses focus. |
| `.style(style)` | `impl Into<text_input::StyleFn>` | `theme::text_input::Default` | Override the style — variant unit structs (`Default`, `Invalid`) or a hand-rolled `Spec`. |

### `Size`

| Variant | Height | Notes |
| --- | --- | --- |
| `Size::Xs` | 24px | Extra-compact controls. |
| `Size::Sm` | 28px | Compact form rows. |
| `Size::Default` | 32px | Standard height. |
| `Size::Lg` | 36px | Larger touch targets. |

### Validation

Apply `theme::text_input::Invalid` with `.style(...)` to show the destructive border and ring, including when the input is unfocused.

File inputs still require a separate file picker. Wrap the input in `field(input(...)).label(...)` to focus it when the user presses the label.
