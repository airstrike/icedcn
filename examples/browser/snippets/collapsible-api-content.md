### Content — `c.content(child)`

Show the child only while open. Closing unmounts it, so a nested disclosure or an uncontrolled input starts from its default on reopening. Put persistent values in the application's state and use controlled widgets when they must survive unmounting.

There is no `force_mount` builder: for always-mounted content, return the child directly in the closure and use `c.is_open()` for your chosen presentation. This corresponds to forcing Radix content to stay mounted; `c.content(...)` is the conditional form. A closed content slot has zero size. Put the gap in the content's top padding when a parent stack would otherwise reserve spacing for that empty slot.

Nova does not prescribe a height transition for Collapsible content. The File Tree example independently animates its chevron rotation.
