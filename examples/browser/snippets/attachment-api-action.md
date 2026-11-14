### AttachmentAction

`attachment::action(content)` returns an icedxdy button. Add an `.on_press(message)` handler and use the usual button builders to change its appearance.

| Property | Type | Default |
| --- | --- | --- |
| Content | `impl Into<Slot>` | Required |
| Size | `Size` | Square, `Xs` |
| Style | Button style | `button::Ghost` |
