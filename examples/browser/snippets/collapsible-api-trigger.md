### Trigger — `c.trigger(child)`

Make a composed child toggle the disclosure. Use `button(...)` for a focusable pointer and keyboard trigger. Enter and Space activate it. The child keeps its own `on_press` message. A root-disabled or child-disabled trigger never toggles.

The handle/`Slot` composition is the native equivalent of `CollapsibleTrigger asChild`; there is no additional button or focus target around the child.
