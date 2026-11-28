### Card

The `Card` component is the root container for card content.

`card(content)` accepts `impl Into<View>` and returns a `Card` builder implementing `Compose`. It keeps arbitrary children lazy, with the card background, foreground/10 border, 14px corners and clipping. The root supplies its outer column gap and vertical padding; a descendant footer removes the bottom padding. A nested Card starts its own layout context.

| Builder | Type | Default |
|---|---|---|
| `.width(width)` | `impl Into<Length>` | content width |
| `.height(height)` | `impl Into<Length>` | content height |
| `.size(size)` | `impl Into<Size>` | `Size::Default` |
| `.spacing(pixels)` | `impl Into<Pixels>` | 16; 12 for `Size::Sm` |
| `.gap(pixels)` | `impl Into<Pixels>` | root spacing |
| `.padding(padding)` | `impl Into<Padding>` | root spacing vertically; 0 below a footer |
| `.clip(clip)` | `bool` | true |
| `.style(style)` | iced container style callback | Card surface |

Use `.width(Fill.max(384))` to fill the available width up to 384px. Use `.padding(0)` for an image-first composition. Explicit column spacing and section padding override the inherited defaults. Keep children as Slots until the application Element boundary; `.into()` composes there.
