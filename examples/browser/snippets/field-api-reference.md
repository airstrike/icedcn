## API Reference

### `field()`

Constructor for a `Field` builder. Wraps any `impl Into<Slot>` (typically an `input(...)`, `checkbox(...)`, `switch(...)`, etc.) with optional label / description / error / card chrome.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl IntoLabel` | `None` | Plain strings auto-style as `text::label`; pre-styled `Text` / `Slot` pass through. Pressing the label activates the field's control. A checkbox, switch or radio responds to activation, and an input takes focus. |
| `.description(description)` | `impl IntoDescription` | `None` | Plain strings auto-style as `text::muted`; pre-styled values pass through. |
| `.error(error)` | `impl IntoError` | `None` | Plain strings auto-style as `text::error`; pre-styled values pass through. Note: the error message does NOT auto-style the inner content widget as invalid — wire `input(...).style(theme::text_input::Invalid)` (or the equivalent) at the call site. |
| `.orientation(orientation)` | `field::Orientation` | `Vertical` | Layout axis. Variants are reachable at module scope (`field::Vertical`, `field::Horizontal`). |
| `.card()` | — | no card | Wrap the field in a rounded, bordered card with 10px padding. The whole card is the label, and it takes the primary tint while its control is checked, read through the `checked` operation. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. Applied to the final rendered element so callers don't need to wrap a single Field in a `column![field].width(N)`. |

### `field::Orientation`

| Variant | Description |
| --- | --- |
| `Vertical` | Label → content → description → error stacked in a Column with 8px gap. This is the default. |
| `Horizontal` | Content on the left, FieldContent column (label / description / error at 2px gap) on the right, in a Row with 8px gap. Alignment is `Center` for label-only and `Start` when description or error is present. |

### Conversion traits

The `.label(...)`, `.description(...)`, `.error(...)` builders all accept any of:

- `&str` / `String` — auto-styled via `text::label(...)`, `text::muted(...)`, `text::error(...)` respectively.
- `text::Text<'a>` — pre-styled, passes through.
- `Slot<'a, Message>` — passes through.

Implemented via `field::IntoLabel`, `field::IntoDescription`, `field::IntoError` traits.

### Grouping fields

Use `field::group([...])` to space related controls, or `field::set(legend, description, controls)` to give them a heading. `field::legend(...)` styles a legend. Place `separator()` between groups. Set `.control_last()` to place a horizontal field’s control after its label and description.

Use `.description_above()` for helper text before a vertical control. `.responsive()` switches to a horizontal layout at a container width of 448 logical pixels. `.disabled(true)` disables all controls inside the field.
