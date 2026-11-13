## API Reference

| Builder | Behavior |
| --- | --- |
| `.visible(n)` | Show up to this many slides together. |
| `.spacing(px)` | Set the gap between visible slides. |
| `.orientation(axis)` | Lay out a horizontal or vertical track. |
| `.default_value(index)` | Start at this zero-based snap. |
| `.value(index)` | Use an application-owned snap index. |
| `.on_value_change(f)` | Receive the requested snap after navigation. |
| `.autoplay(duration)` | Advance on a timer while visible and unhovered. Manual navigation stops autoplay. |
| `.controls(previous, next)` | Supply custom content for both navigation buttons. |

Tab into the carousel and navigate with the arrow keys, press the previous and next buttons, or swipe along the track.
