## API Reference

### `popover(trigger, content)`

The trigger closure supplies the anchor. The content closure builds its panel each time it opens. Both receive a `popover::Handle` and can publish your application’s messages.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled popover starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the popover: open exactly when the app says so. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state when the popover opens or is dismissed. |
| `.side(side)` | `popover::Side` | `Bottom` | The side of the trigger the panel prefers; it flips when there is no room. |
| `.align(align)` | `popover::Align` | `Center` | `Start`, `Center` or `End` along the trigger. |
| `.side_offset(px)` | `f32` | `4.0` | The gap between the trigger and the panel. |
| `.align_offset(px)` | `f32` | `0.0` | Moves a start- or end-aligned panel inward. |
| `.width(px)` | `f32` | `288.0` | The panel's width. |

### `popover::Handle`

| Method | Description |
| --- | --- |
| `p.trigger(child)` | Pressing `child` opens or closes the popover. |
| `p.close(child)` | Pressing `child` closes it. The child also publishes its own `.on_press` message if configured. |
| `p.is_open()` | Whether the popover is open. |

Opening the panel focuses its first control and selects the text in a focused input. Tab and Shift+Tab move between controls. Escape or a press outside dismisses the panel. An outside press can also activate the underlying page.

Use `popover::header(title, description)` for a title and supporting text.
