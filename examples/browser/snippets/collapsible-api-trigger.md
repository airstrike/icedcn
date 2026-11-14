### Trigger — `c.trigger(child)`

Make a composed child toggle the disclosure. Use `button(...)` for a focusable pointer and keyboard trigger. Enter and Space activate it. The child keeps its own `on_press` message. A root-disabled or child-disabled trigger never toggles.

`c.trigger(child)` retains the child’s input behavior and focus target. An interactive child can still publish its own message.
