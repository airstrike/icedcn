### AspectRatio

The `AspectRatio` component displays content within a desired ratio.

| Argument or builder | Type | Default | Required |
| --- | --- | --- | --- |
| `content` | `impl Into<Slot>` | — | Yes |
| `ratio` | `f32` (width divided by height) | — | Yes |
| `.width(width)` | `impl Into<Length>` | `Length::Fill` | No |
| `.max_width(width)` | `impl Into<Pixels>` | Unbounded | No |

A non-positive or non-finite ratio falls back to a square. Apply backgrounds, rounded corners and image fitting to the child.

[Read the aspect_ratio implementation](https://github.com/airstrike/icedxdy/blob/HEAD/widget/src/aspect_ratio.rs).
