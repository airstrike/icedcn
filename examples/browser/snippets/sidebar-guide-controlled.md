## Controlled Sidebar

Use controlled mode to share the open state with other parts of the application. Update the stored value when the message arrives.

```rust
sidebar::provider(|s| layout(s))
    .open(state.sidebar_open)
    .on_open_change(Message::SidebarOpenChanged)
```
