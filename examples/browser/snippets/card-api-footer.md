### CardFooter

`card::footer(child)` accepts a `Slot` conversion and returns a `Footer` builder implementing `Compose` with a top separator, muted/50 background, lower rounded corners and root spacing on all sides (16px outside a Card). Use `card::footer_with_spacing(child, spacing)` to set that padding. Supply a row or column to arrange its controls. Its composition preserves inherited properties until the footer is composed.

`card::footer_row(children)` accepts Slots and uses an 8px horizontal gap. The footer adds no application state or automatic submit behavior.
