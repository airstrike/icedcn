## API Reference

`scroll_area(content)` creates a native vertical viewport. Its content can contain interactive widgets. Use `.width(...)` and `.height(...)` to bound the viewport.

| Builder | Effect |
| --- | --- |
| `.direction(Direction::Horizontal(scroll_area::bar()))` | Scroll left and right with the themed scrollbar. |
| `.direction(Direction::Both { vertical, horizontal })` | Provide a scrollbar for each axis. |
| `.id(id)` | Target scroll operations at a particular viewport. |
| `.on_scroll(f)` | Publish a message with the new `Viewport` when the viewport changes. Read its offset with `absolute_offset()`. |
| `.visibility(Visibility::Hover)` | When the scrollbars show: `Hover` (the default) while the pointer is over the area, `Scroll` while it scrolls, `Auto` when the content overflows, `Always`. |
| `.hide_delay(duration)` | How long an idle scrollbar stays before it fades out in `Hover` and `Scroll`. 600ms by default. |
