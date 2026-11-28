### CardAction

The `CardAction` component places content in the top-right of the header (for example, a button or a badge).

`card::action(content)` returns the supplied `View`. Pass it to `header.action(...)`, which owns its placement. A button publishes the app message provided to `.on_press(...)`; handle that message in your update function.
