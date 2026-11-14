## Composition

Pass the target and menu contents as separate closures to `context_menu`. Use the `menu` helpers to place commands, groups and nested menus in the popup.

```text
context_menu(trigger, content)
├── trigger area
└── column!
    ├── menu::group(items)
    │   ├── menu::label(label)
    │   └── menu::item(label)
    ├── menu::separator()
    ├── menu::checkbox_item(label, checked, on_toggle)
    ├── menu::radio_item(label, value, selected, on_select)
    └── menu::sub(label, items)
```
