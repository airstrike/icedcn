### AttachmentGroup

Use `attachment::group(cards)` for a horizontal row with 12 px gaps and 4 px vertical padding. Each card keeps its width. Wheel and arrow scrolling settle at card starts with 4 px scroll padding. Home and End reach the ends, and focusing an offscreen action reveals it.

The scrollbar is hidden. The strip fades its overflowing edges over the surrounding canvas; the fade is the smaller of 12% of its width and 40 px. Set `.fade_to(...)` when the canvas differs from the theme background, as the installed examples do.

| Builder | Type | Default |
| --- | --- | --- |
| `attachment::group(cards)` | Iterator of `Slot` | Required |
| `.fade_to(...)` | `fn(&Theme) -> Color` | Theme background |
