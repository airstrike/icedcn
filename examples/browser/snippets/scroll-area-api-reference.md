## API Reference

`scroll_area(content)` creates a native vertical viewport. Its content can contain interactive widgets. Use `.width(...)` and `.height(...)` to bound the viewport.

| Builder | Effect |
| --- | --- |
| `.direction(Direction::Horizontal(scroll_area::bar()))` | Scroll left and right with the Nova scrollbar. |
| `.direction(Direction::Both { vertical, horizontal })` | Provide a scrollbar for each axis. |
| `.id(id)` | Target scroll operations at a particular viewport. |
| `.on_scroll(callback)` | Receive the viewport and its offset as it moves. |
