## API Reference

| API | Behavior |
| --- | --- |
| `item(key, content).anchor(true)` | Mark a message or event as the start of a turn. |
| `.default_scroll_position(position)` | Open at Start, End or LastAnchor. |
| `.auto_scroll(true)` | Follow output at the live edge; scrolling away releases it. |
| `.scroll_margin(px)` | Reserve room between the top and an anchor. |
| `.previous_item_peek(px)` | Leave part of the preceding exchange visible. |
| `.preserve_scroll_on_prepend(true)` | Preserve the current reading position when earlier rows arrive. |
| `.id(id)` | Identify the viewport for focus and scrolling. |
| `scroller::scroll_to(id, target, animation)` | Return this task from update to move to Start, End or a keyed item. Choose Instant or Smooth for each jump. |
| `.on_reader_change(Message::Reader)` | Publish a message with the visible keys, current anchor and scroll availability. |
| `.end_button(visible, icon, message)` | Animate a floating end button above the viewport and publish its press message. |
| `.fade_to(background)` | Fade the bottom of the viewport into a solid background while more content remains. |
