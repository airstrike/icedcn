### CardFooter

The `CardFooter` component is used for actions and secondary content at the bottom of the card.

`card::footer(child)` accepts a `Slot` conversion and returns a `Slot` with a top separator, muted/50 background, lower rounded corners and 16px padding on all sides. Use `card::footer_with_spacing(child, spacing)` to set that padding. Supply a row or column to arrange its controls. Its composition preserves inherited properties until the footer is composed.

`card::footer_row(children)` accepts iced Elements and uses an 8px horizontal gap. The footer adds no application state or automatic submit behavior.
