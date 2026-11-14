### Card

The `Card` component is the root container for card content.

`card(content)` accepts an iced `Element` conversion and returns a stock iced `Container` with the card background, foreground/10 border, 14px corners and clipping. It adds no layout padding or section gaps. Compose those explicitly with a column as in Usage.

| Builder | Type | Default |
|---|---|---|
| `.width(width)` | `impl Into<Length>` | content width |
| `.height(height)` | `impl Into<Length>` | content height |
| `.padding(padding)` | `impl Into<Padding>` | 0 |
| `.clip(clip)` | `bool` | true |
| `.style(style)` | iced container style callback | Card surface |

Use `.width(Fill.max(384))` for the source's full-width card with a 384px maximum. Root `size` and automatic spacing inheritance are not currently exposed; the Size and Spacing recipes apply the corresponding values explicitly.
