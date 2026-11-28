### Field

`field(control)` accepts anything that converts to a `View`.

| Builder | Effect |
| --- | --- |
| `.orientation(Horizontal)` | Places the control before the label and helper text. Vertical is the default. |
| `.control_last()` | Places the horizontal control after the text. |
| `.responsive()` | Stacks content below a container width of 448 px and arranges it in a row at wider sizes. |
| `.width(width)` | Sets the field’s outer width. Give an input or select `.width(Fill)` to stretch its control. |
| `.disabled(true)` | Disables the field’s controls. |
| `.card()` | Makes the entire surface activate a checkable control and tints it when checked. |
