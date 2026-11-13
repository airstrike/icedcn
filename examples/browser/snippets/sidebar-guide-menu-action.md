## SidebarMenuAction

Place a separate action beside a navigation button in the same menu item. For example, use the trailing action to open a project menu.

```rust
sidebar::menu_action(icon::ellipsis()).on_press(Message::ProjectActions)
```
