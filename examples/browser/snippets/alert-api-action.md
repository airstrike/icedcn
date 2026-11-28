### AlertAction

Use `.action(action)` to position a button or other action element in the top-right corner. It accepts `impl Into<View>` and defaults to `None`. The content is inset 8px from the inside of the border.

The child owns its interaction and publishes its own messages. Alert forwards inherited disabled state to the action. Clicking an action does not dismiss the alert.
