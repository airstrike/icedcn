## Accessibility

Focus the transcript to use the arrow keys, Page Up, Page Down, Home and End. Space advances by a page, and Shift+Space moves back. Wheel and touch gestures use the native scroll area. The Scroll Status example shows the directions the reader can still travel.

Use `Animation::Instant` for jump commands when the reader prefers reduced motion, and leave new rows unanimated with `motion(row).animate(false)`. If the surrounding application announces incoming messages, group streamed chunks into one announcement per reply.
