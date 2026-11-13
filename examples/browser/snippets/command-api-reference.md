## API Reference

| Builder | Behavior |
| --- | --- |
| `command(items)` | Filter a list of commands as the user types. |
| `.placeholder(text)` / `.empty(text)` | Customize the input and empty result text. |
| `command::item(label)` | Create a selectable action. |
| `.group(heading)` / `.icon(icon)` / `.shortcut(text)` | Organize and present actions in the list. |
| `.disabled(bool)` / `.on_select(message)` | Disable a command or handle its selection. |

Arrow keys move through enabled matches. Home and End jump to the first and last. Enter runs the highlighted action.
