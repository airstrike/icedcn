## API Reference

| Builder | Behavior |
| --- | --- |
| `item(key, content).anchor(true)` | Mark a message or event as the start of a turn. |
| `.default_scroll_position(position)` | Open at Start, End or LastAnchor. |
| `.auto_scroll(true)` | Follow output at the live edge; scrolling away releases it. |
| `.scroll_margin(px)` | Reserve room between the top and an anchor. |
| `.previous_item_peek(px)` | Leave part of the preceding exchange visible. |
| `.preserve_scroll_on_prepend(true)` | Preserve the current reading position when earlier rows arrive. |
| `.command(Some(command))` | Request Start, End or a stable item. An early item request waits for the transcript to mount. |
| `.smooth_commands(true)` | Animate explicit jumps using the native scroll animation. |
| `.on_reader_change(callback)` | Receive visible keys, current anchor and scroll availability. |
