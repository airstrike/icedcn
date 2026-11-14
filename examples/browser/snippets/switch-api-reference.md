## API Reference

### `switch()`

Constructor for an off, uncontrolled `Switch`. The value lives in the widget tree unless the app controls it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_checked(bool)` | `bool` | `false` | Initial state of an uncontrolled switch. |
| `.checked(bool)` | `bool` | uncontrolled | Control the switch: it always shows the given state. |
| `.on_checked_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the new checked state on each toggle in either mode. |
| `.disabled(bool)` | `bool` | `false` | Disable the switch. A disabled ancestor disables it too. |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.style(style)` | `impl Into<switch::StyleFn>` | `theme::switch::Default` | Variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. |

### Animated thumb slide

The thumb's position is animated via the vendored `Toggler`'s tree state — no caller bookkeeping needed.

### Labels

Inside `field(switch()).label("...")`, pressing the label toggles the switch.

Use `.size(switch::Size::Sm)` for the compact switch. The default size is `switch::Size::Default`.
