## Composition

`command(items)` composes a search input and scrollable results. Each `command::item` supplies a label with optional icon, shortcut hint, and selection message. Consecutive `.group(heading)` values create headings and separators; groups without matches disappear during filtering. `.empty(text)` supplies the empty state.

Use `command::dialog(trigger, || command(items))` for a modal palette. Its trigger receives the regular dialog handle, and the content closure builds the same command list used inline.
