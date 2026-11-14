## API Reference

### `dropdown_menu(trigger, content)`

A `Popover` holding a menu: as wide as its trigger (at least 128px) unless given a `.width(..)`, with 4px padding, and modal, so the page ignores the pointer while it is open and a press anywhere outside closes it. The closures receive a `dropdown_menu::Handle` (`m.trigger(child)`, `m.close(child)`, `m.is_open()`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled menu starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the menu. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state. |
| `.side(side)` | `Side` | `Bottom` | The side of the trigger the menu prefers. |
| `.align(align)` | `Align` | `Start` | `Start`, `Center` or `End`. |
| `.side_offset(px)` | `f32` | `4.0` | The gap between the trigger and the menu. |
| `.width(px)` | `f32` | trigger width | A fixed menu width. |

### `menu`

| Part | Description |
| --- | --- |
| `menu::item(content)` | `DropdownMenuItem`: `.on_select(message)`, `.shortcut(str)`, `.disabled(bool)`, `.inset(bool)`, `.style(menu::Destructive)`. Selecting it closes the menu. |
| `menu::checkbox_item(content, checked, on_checked_change)` | `DropdownMenuCheckboxItem`, with a trailing check. |
| `menu::radio_item(content, value, selected, on_select)` | `DropdownMenuRadioItem`, checked while `selected` holds `value`. |
| `menu::label(text)` | `DropdownMenuLabel`. |
| `menu::separator()` | `DropdownMenuSeparator`. |
| `menu::group(children)` | `DropdownMenuGroup`. |
| `menu::sub(content, items)` | `DropdownMenuSub`: opens `items` (like `group`'s) to the right while hovered; selecting inside closes the whole menu. |

Use `menu_group![...]` and `menu_sub![trigger, ...]` to combine different item types. `menu::caption(icon, label)` aligns a leading icon with its label.

Press Enter, Space or Down Arrow on the focused trigger to open the menu at its first item. Pointer opening leaves the items unselected until you point at one or use the keyboard. Up and Down Arrow move between enabled items. Home and End jump to the first and last. Type a label prefix to focus a match. Right Arrow opens the focused submenu, and Left Arrow returns to its parent.
