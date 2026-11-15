## API Reference

| API | Behavior |
| --- | --- |
| `item(key, content).anchor(true)` | Mark a message or event as the start of a turn. |
| `.default_scroll_position(position)` | Choose Start, End or LastAnchor. The default is LastAnchor. |
| `.auto_scroll(true)` | Follow new output at the live edge. Following is off by default. |
| `.scroll_margin(px)` | Inset an anchor from the padded top edge. Defaults to 16px. |
| `.previous_item_peek(px)` | Leave part of the preceding exchange visible above a new anchor. Defaults to 0px. |
| `.preserve_scroll_on_prepend(true)` | Keep the same visible row and offset when earlier rows arrive. On by default. |
| `.height(length)` | Fill the available height by default, or set an explicit viewport height. |
| `.spacing(px)` and `.padding(px)` | Set the row gap and content inset. Defaults are 12px and 16px. |
| `.id(id)` | Identify the viewport for focus and scrolling. |
| `scroller::scroll_to(id, target, animation)` | Return this task from update to move to Start, End or a keyed item. Choose Instant or Smooth for each jump. |
| `.on_reader_change(Message::Reader)` | Publish a message with the visible keys, current anchor and scroll availability. |
| `.end_button(visible, icon, message)` | Animate a floating end button above the viewport and publish its press message. |
| `.fade_to(background)` | Fade the bottom of the viewport into a solid background while more content remains. |

`Reader` reports `can_scroll_up` and `can_scroll_down` as booleans. Its `current_anchor: Option<u64>` identifies the most recent anchor at or above the reading line. Its `visible: Vec<u64>` contains the keys intersecting the reading area below that line.
