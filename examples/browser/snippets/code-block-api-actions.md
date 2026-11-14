### Action controls

`.actions(...)` holds any number of controls. Without a title, it floats at the upper end of the content and appears on hover or keyboard focus.

Floating controls fade over 150ms. The current renderer fades inherited text and theme-derived colors; explicit child colors and artwork remain opaque during the transition.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.copyable(bool)` | `bool` | `false` | Append a Copy button for text, diff or ANSI source. |
| `.actions(actions)` | `impl Into<Slot>` | absent | Usually a row of ghost icon buttons with a 4px gap. |
