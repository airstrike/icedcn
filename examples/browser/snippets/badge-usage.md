## Usage

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::badge;

fn example() -> Element<'static, (), Theme> {
    badge("Badge").style(badge::Default).into()
}
```

Set `.style(...)` to `badge::Default`, `badge::Secondary`, `badge::Destructive`, `badge::Outline`, `badge::Ghost`, or `badge::Link`. Use `badge![icon, "Label"]` for inline content with a 4px gap, and `.icon_start()` or `.icon_end()` for the corresponding inset. Add `.on_press(Message::OpenLink)` to publish an application message.
