## API Reference

| Builder | Effect |
| --- | --- |
| `.orientation(axis)` | Select a horizontal or vertical split. |
| `.default_value(fraction)` | Set the initial share of the first pane between zero and one. Defaults to one half. |
| `.value(fraction)` | Control the split from application state. |
| `.on_value_change(f)` | Publish a message with the new fraction when the divider moves by dragging or keyboard input. |
| `.with_handle()` | Add a visible grip to the divider. |

Tab to the divider and use the arrow keys to resize by one percent. Hold Shift for ten percent. Home and End move to the limits.
