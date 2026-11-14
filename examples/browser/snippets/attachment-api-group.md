### AttachmentGroup

`attachment::group(cards)` takes an iterator of card slots. It arranges the cards in a horizontal scroll area, with 12 px between cards. Set an explicit width on each horizontal card to preserve its intended width in the group.

| Argument | Type | Default |
| --- | --- | --- |
| `cards` | `impl IntoIterator<Item = Slot>` | Required |
