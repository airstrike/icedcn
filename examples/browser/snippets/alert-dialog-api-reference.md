### size

Use the `.size(..)` builder on `AlertDialog` to control the size of the alert dialog. It accepts the following values:

| Builder | Type | Default |
| --- | --- | --- |
| `.size(size)` | `Size::Default` or `Size::Sm` | `Size::Default` |

[Read the alert_dialog implementation](https://github.com/airstrike/icedxdy/blob/HEAD/widget/src/alert_dialog.rs).

**`alert_dialog()`**

`alert_dialog(trigger, content)` is a dialog without a corner close button that a press outside does not dismiss: its cancel and action buttons and `Escape` close it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Default` | `Sm`: a 320px panel, centered header, footer buttons in equal halves. |
| `.default_open(open)` | `bool` | `false` | Whether an uncontrolled dialog starts open. |
| `.open(open)` | `bool` | - | Control the dialog. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | The message published with the new open state. |

**The handle**

| Method | Description |
| --- | --- |
| `a.trigger(child)` | Pressing `child` opens the dialog. |
| `a.header(title, description)` | Set the title and description. Add a media tile with `.media(icon)` and customize its style with `.media_style(...)`. |
| `a.footer(buttons)` | A muted band below a border. Give the column holding the header and footer `.width(Fill)` so the band spans the panel. |
| `a.cancel(button)` | The button closes the dialog. |
| `a.action(button)` | The button closes the dialog and still publishes its `on_press`. |
