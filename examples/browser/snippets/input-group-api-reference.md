## API Reference

### `input_group()`

`input_group(control)` puts an `input(..)` or a `textarea(..)` and its addons in one bordered field. The group paints the Input's border, background and focus ring (the ring shows while the control is focused) and strips the control's own.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.addon(addon)` | `input_group::Addon` | - | Add an addon. |
| `.disabled(disabled)` | `bool` | `false` | Disable the control and dim the group. |
| `.style(style)` | `impl Into<text_input::StyleFn>` | `text_input::Default` | `text_input::Invalid` paints the invalid surface. |

### `input_group::addon(content)`

`InputGroupAddon`: muted 14px medium-weight content, icons at 16px. `.align(..)` places it: `InlineStart` (default), `InlineEnd`, `BlockStart` or `BlockEnd`. Use `.divider()` on a block addon to give an editor toolbar a full-width separator.

### `input_group::text(..)` / `input_group::button(..)`

`InputGroupText` (muted 14px text) and `InputGroupButton` (a ghost `Xs` button; chain `.square()` for a square icon button).

### Input and textarea controls

Use `input_group(input(placeholder))` for one line and `input_group(textarea(placeholder))` for multiline text. Both accept uncontrolled defaults or an application-owned value with an input handler. Set the control’s text size with `.text_size(..)`. A textarea grows with its text and accepts `.min_height(..)`. The grouped surface owns the shared border and focus ring.
