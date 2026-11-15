### Trigger — `c.trigger(child)`

Make a composed child toggle the disclosure. Use `button(...)` for a focusable pointer and keyboard trigger. Enter and Space activate it. The child keeps its own `on_press` message. A root-disabled or child-disabled trigger never toggles.

`c.trigger(child)` retains the child’s input behavior and focus target. An interactive child can still publish its own message.

Place `c.indicator()` inside the trigger for a chevron that rotates from right to down over 150 ms. It inherits the trigger's text color. Use `.size(14)` to set its square bounds and `.duration(Duration::ZERO)` for an immediate turn. The indicator follows the disclosure state without application messages.
