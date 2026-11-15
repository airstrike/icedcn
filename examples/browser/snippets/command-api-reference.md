## API Reference

| Builder | Default | Behavior |
| --- | --- | --- |
| `command(items)` | Inline | Searchable items in source order, with local query and selection. |
| `.placeholder(text)` | `Type a command or search...` | Search input hint. |
| `.empty(text)` | `No results found.` | Message when no items match. |
| `.width(length)` | `Fill` | Palette width. |
| `.limit(count)` | All | Maximum matches displayed. |
| `.embedded(field)` | None | Supply a search-field composition around the provided `Input`. |
| `command::item(label)` | Enabled | Item label and case-insensitive search text. All query words must match. |
| `.group(heading)` | None | Heading for consecutive items; groups are separated automatically. |
| `.icon(function)` | None | Leading icon, supplied as a function returning an iced text widget. |
| `.shortcut(text)` | None | Visual keyboard hint. Does not register a shortcut. |
| `.disabled(bool)` | `false` | Keep the row visible but prevent selection. |
| `.on_select(message)` | None | Publish an application message on click or Enter. |
| `command::dialog(trigger, content)` | Closed | Modal palette with search autofocus, top-third placement, and no close button. |

`command::dialog` returns a regular `dialog::Dialog`. Use `.default_open(true)` for an initially open uncontrolled menu, or `.open(state).on_open_change(Message::OpenChanged)` for controlled state. Handle `OpenChanged` in `update` by assigning the new boolean.

Arrow keys move through enabled matches without wrapping. Home and End select the first and last enabled match. Keyboard selection scrolls into view; moving the pointer over a row changes the selection. Enter publishes the highlighted item's message. Escape or a click outside dismisses a dialog and restores trigger focus.
