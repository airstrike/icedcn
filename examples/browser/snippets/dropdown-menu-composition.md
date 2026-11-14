## Composition

Wrap the opening control with the handle’s `trigger`. Build the popup from `menu` items, groups, separators and submenus in the content closure.

```text
dropdown_menu(trigger, content)
├── m.trigger(button)
└── menu_group![...]
    ├── menu::label(label)
    ├── menu::item(label)
    ├── menu::separator()
    ├── menu::checkbox_item(label, checked, on_toggle)
    ├── menu::radio_item(label, value, selected, on_select)
    └── menu_sub![label, ...]
```
