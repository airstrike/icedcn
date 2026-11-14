### AttachmentAction

`attachment::action(content)` returns an icedxdy button. Use `.on_press(message)` to publish a message when the button is activated. Style it with the usual button builders.

| Property | Type | Default |
| --- | --- | --- |
| Content | `impl Into<Slot>` | Required |
| Size | `Size` | Square, `Xs` |
| Style | Button style | `button::Ghost` |
