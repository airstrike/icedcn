## Accessibility

- Content participates in native keyboard focus traversal by default. `.focusable(false)` removes its stop; `.id(...)` identifies it for focus operations.
- Arrows and Home/End scroll focused content. Page Up/Down and Space scroll vertically when content is not collapsed.
- Floating controls are reachable by keyboard and appear when content or an action is focused.
- Line numbers and diff markers are separate, nonselectable decoration. Added and removed lines include `+` and `−` as well as a tint.
- Copy controls have a tooltip labelled “Copy” and display a check with “Copied” after successful clipboard completion. A repeated copy restarts the two-second feedback timer.
- The expansion button changes its label from “Show more” to “Show less” while the full listing is visible.
