## SidebarMenuButton

Use `.active(true)` for the current destination, `.on_press(message)` to navigate, and `.collapsed(true)` when showing the icon rail. Each row uses the sidebar accent palette.

```rust
sidebar::menu_button("Dashboard").active(true).on_press(Message::Dashboard)
```
