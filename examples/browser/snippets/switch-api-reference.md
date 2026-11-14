## API Reference

`switch()` constructs an off, uncontrolled switch. The value lives in the widget tree unless the app controls it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_checked(bool)` | `bool` | `false` | Initial state of an uncontrolled switch. |
| `.checked(bool)` | `bool` | uncontrolled | Control the switch: it always shows the given state. |
| `.on_checked_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the new checked state on each toggle in either mode. |
| `.disabled(bool)` | `bool` | `false` | Prevent interaction and skip the switch during Tab traversal. A disabled ancestor also disables it. |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.size(size)` | `switch::Size` | `Default` | Choose a 32 × 18.4 px track or a 24 × 14 px small track. |
| `.style(style)` | `impl Into<theme::switch::StyleFn>` | `switch::Default` | Variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. |

The thumb animates between its checked and unchecked positions.

### Labels and keyboard input

Inside `field(switch()).label("...")`, pressing the label toggles the switch. Add `.card()` to make the whole field clickable. Tab moves focus to an enabled switch. Enter or Space toggles the focused switch. Moving off the pointer target before release cancels a press.
