### BubbleReactions

Displays overlapped reactions for a bubble. Use `.reactions(content, align, side)`; supply the reaction content, horizontal alignment, and vertical side.

| Argument or builder | Type | Default | Behavior |
|---|---|---|---|
| `content` | `impl Into<Slot<'a, Message>>` | Required | Reaction text, a row, a button, tooltip or popover. |
| `side` | `Vertical` | Required | `Top` or `Bottom` edge. |
| `align` | `Horizontal` | Required | `Left` or `Right`, 12px from the bubble edge. |
| `.reaction_padding(padding)` | `impl Into<Padding>` | `[2, 6]` | Use zero for reactions containing buttons. |

The reaction is translated by 75% of its measured height beyond the edge; a 3px card-colored ring sits outside it. It does not add height to the conversation row. Leave vertical space between bubbles so overlapping reactions do not collide. Native labels and text alternatives are covered under Accessibility.
