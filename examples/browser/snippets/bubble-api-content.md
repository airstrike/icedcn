### BubbleContent

The bubble content wrapper is the content `Slot` passed to `bubble(content)`, so strings, rich text, columns and local components compose directly.

| Builder | Type | Default | Behavior |
|---|---|---|---|
| `bubble(content)` | `impl Into<Slot<'a, Message>>` | Required | Inherits 14px text with 22.75px line height unless explicitly set. |
| `.padding(padding)` | `impl Into<Padding>` | `[10, 12]`; zero for Ghost | Space inside the reserved one-pixel border. |
| `.radius(radius)` | `impl Into<Radius>` | Nova 3XL (22px); zero for Ghost | Corner radius. |
| `.interactive(true)` | `bool` | `false` | Native equivalent of making `BubbleContent` a button with `asChild`. |

The content fits its intrinsic width, capped at 80% of the row. Ghost permits the full row. Explicit child sizes and inherited disabled state follow Compose rules.
