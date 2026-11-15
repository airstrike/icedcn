## Stream text without replacing rows

Append received text to the existing message in `update`, then rebuild its bubble with the same key. For example, an application can handle a `Delta(id, text)` message by finding the entry with that id and calling `push_str` on its body. Create a new id only when a new message begins.

Use `.auto_scroll(true)` on the scroller when new output should follow the live edge. It is off by default. `.preserve_scroll_on_prepend(true)` is on by default and preserves the reader's position when older messages arrive. Do not repeatedly issue a scroll-to-end task for every token: give the reader room to inspect earlier content.

`.on_reader_change(...)` reports visible keys, the current anchor, and whether more content can be scrolled into view. Use that state to decide when to show a return-to-latest action. The [scrolling API](icedcn://component/message-scroller#api-reference) includes explicit keyed jumps and smooth or instant movement.

Transport and model selection belong to your application. Feed network or local-model output into iced messages; the transcript widgets present the resulting state.
