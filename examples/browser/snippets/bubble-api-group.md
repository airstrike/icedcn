### BubbleGroup

Groups consecutive bubbles from the same sender.

| Constructor or builder | Type | Default | Behavior |
|---|---|---|---|
| `bubble::group(items)` | `impl IntoIterator<Item = Slot<'a, Message>>` | Required | A full-width column of bubbles. |
| `.spacing(gap)` | `impl Into<Pixels>` | `8` | Vertical gap between adjacent bubbles. |

Set alignment on the bubbles themselves. The returned `Column` retains its normal layout builders, and surrounding Message alignment still cascades to its children.
