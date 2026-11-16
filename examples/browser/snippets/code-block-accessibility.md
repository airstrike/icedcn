## Accessibility

- The content always reads left to right.
- The content is a keyboard tab stop, so keyboard users can focus it and scroll it with the arrow keys, Page Up and Page Down, Home and End, or Space. Pass `.focusable(false)` to remove the tab stop.
- Floating actions show when focus is inside the block, not only on hover, so keyboard users can reach them.
- Line numbers and diff markers are not part of a selection.
- Diff lines carry a `+` or `−` marker as well as a tint, so the change does not rely on color.
- `code_block::copy_button` has a "Copy" tooltip, which reads "Copied" after a successful copy.
- Give a wrap toggle `.pressed(wrap)` so its state shows.
- The Show More button reads "Show less" while the content is expanded.
