## Usage

```rust
use icedcn::widget::badge;

fn example<'a>() -> badge::Badge<'a, ()> {
    badge("Badge").style(badge::Default)
}
```

Set `.style(...)` to `badge::Default`, `badge::Secondary`, `badge::Destructive`, `badge::Outline`, `badge::Ghost`, or `badge::Link`. Use `badge![icon, "Label"]` for inline content with a 4px gap, and `.icon_start()` or `.icon_end()` for the corresponding inset. Add `.on_press(Message::OpenLink)` to publish an application message.
