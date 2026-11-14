## API Reference

[Read the carousel implementation](https://github.com/airstrike/icedcn/blob/HEAD/widget/src/carousel.rs).

| API | Default | Behavior |
| --- | --- | --- |
| `carousel(count, slide)` | Required | Keep all slides mounted, indexed from zero. Factory returns `impl Into<Slot>`. |
| `.width(length)` | `Fill` | Track width; horizontal controls reserve another 48px on each side. |
| `.height(pixels)` | Natural slide height | Track viewport height; vertical controls reserve 48px above and below. |
| `.visible(n)` | `1` | Equal slide bases; clamps to at least one. |
| `.spacing(px)` | `16` | Nonnegative gutter between cells. |
| `.orientation(axis)` | `Horizontal` | Horizontal or vertical layout and dragging. |
| `.align(Alignment)` | `Center` | Start, center or end alignment; non-looping endpoints stay contained. |
| `.looping(bool)` | `false` | Wrap movement when there are more slides than fit in the viewport. |
| `.default_value(index)` | `0` | Seed uncontrolled selection once, clamped to the available snaps. |
| `.value(index)` | Uncontrolled | Display the model's snap; callbacks request changes without overriding it. |
| `.on_value_change(f)` | None | Publish the complete requested snap index. A missing handler does not disable navigation. |
| `.disabled(bool)` | `false` | Disable controls, dragging, keyboard and timer; ancestor disabling also applies. |
| `.autoplay(Duration)` | Off | Scheduled advances; zero disables it. Hover pauses; dragging or slide focus stops it. |
| `.stop_on_hover(bool)` | `false` | Stop autoplay when the pointer enters the carousel. |
| `.caption(bool)` | `false` | Optional built-in snap label. Use a Component and callback for application-owned text. |
| `.controls(previous, next)` | chevron icons | Factories for button content, preserving inherited properties and child messages. |
| `.snap_count()` | Derived | Number of selectable snap positions; can differ from the slide count. |

Tab to a navigation button, use Left/Right to move in either orientation, or activate a focused button with Enter/Space. Pointer and touch drag the clipped track. Endpoint controls disable when looping is off. Resizing changes geometry without replacing slides or moving their child state to another index.
