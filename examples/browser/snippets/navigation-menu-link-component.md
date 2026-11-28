# Link Component

Use an application message to navigate to a screen or open a URL.

```rust
use icedcn::widget::navigation_menu;

navigation_menu::link(
    "Documentation",
    Message::OpenUrl("https://github.com/airstrike/icedcn"),
)
```
