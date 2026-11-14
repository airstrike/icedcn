### Content — `c.content(child)`

Show the child only while open. Closing unmounts it, so a nested disclosure or an uncontrolled input starts from its default on reopening. Put persistent values in the application's state and use controlled widgets when they must survive unmounting.

For content that must stay mounted, return the child directly and use `c.is_open()` to choose its presentation. A closed `c.content(...)` slot has zero size. Put the gap in the content’s top padding if a parent column would reserve space for the empty slot.

Collapsible opens and closes immediately. The File Tree example animates its chevron rotation separately.
