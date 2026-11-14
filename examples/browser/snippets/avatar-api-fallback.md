### AvatarFallback

The `AvatarFallback` component displays a fallback when the image fails to load.

| Rust API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.fallback(content)` | `impl Into<Slot>` | empty | Visible without an image and during pending/failed decoding; hidden when the image is ready. |

Fallback content inherits 14px text (12px for small avatars), muted foreground, and nonselectable text. Explicit child settings take precedence. Parent disabled state reaches controls placed in this slot.
