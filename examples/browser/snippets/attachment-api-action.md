### AttachmentAction

`attachment::action(content)` returns an icedcn button. Use `.on_press(message)` to publish a message when the button is activated. Style it with the usual button builders.

| Property | Type | Default |
| --- | --- | --- |
| Content | `impl Into<View>` | Required |
| Size | `Size` | Square, `Xs` |
| Style | Button style | `button::Ghost` |
