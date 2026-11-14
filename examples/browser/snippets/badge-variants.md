```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::{badge, row};

fn example() -> Element<'static, (), Theme> {
    row![
        badge("Default"),
        badge("Secondary").style(badge::Secondary),
        badge("Destructive").style(badge::Destructive),
        badge("Outline").style(badge::Outline),
        badge("Ghost").style(badge::Ghost),
    ]
    .spacing(8)
    .wrap()
    .into()
}
```
