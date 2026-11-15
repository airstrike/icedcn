## API Reference

| Builder | Behavior |
| --- | --- |
| `command(items)` | Inline searchable actions. |
| `.placeholder(text)` | Search input hint. |
| `.empty(text)` | Empty result message. |
| `.width(length)` | Palette width; defaults to `Fill`. |
| `.limit(count)` | Maximum displayed matches. |
| `.embedded(field)` | Compose a custom search field. |
| `command::item(label)` | Label and search text. |
| `.group(heading)` | Group consecutive actions. |
| `.icon(function)` | Leading iced text icon. |
| `.shortcut(text)` | Visual keyboard hint. |
| `.disabled(bool)` | Prevent selection. |
| `.on_select(message)` | Handle click or Enter. |
| `command::dialog(trigger, content)` | Modal command palette. |

The default placeholder is "Type a command or search..." and the empty message is "No results found." The list keeps every match in source order unless `.limit(count)` is set. Filtering ignores case and requires all query words to match the label. Items start enabled with no group, icon, shortcut, or selection message. Shortcut hints do not register application shortcuts.

`command::dialog` starts closed, focuses search on opening, positions the panel at the top third of the window, and hides the close button. It returns a regular `dialog::Dialog`: use `.default_open(true)` for an initially open uncontrolled menu, or `.open(state).on_open_change(Message::OpenChanged)` for controlled state. Handle `OpenChanged` in `update` by assigning the new boolean.

Arrow keys move through enabled matches without wrapping. Home and End select the first and last enabled match. Keyboard selection scrolls into view; moving the pointer over a row changes the selection. Enter publishes the highlighted item's message. Escape or a click outside dismisses a dialog and restores trigger focus.
