```rust
use iced::Element;
use icedxdy::Theme;
use icedxdy::widget::{badge, row, spinner};

fn example() -> Element<'static, (), Theme> {
    row![
        badge![spinner().size(12.0), "Deleting"]
            .icon_start()
            .style(badge::Destructive),
        badge!["Generating", spinner().size(12.0)]
            .icon_end()
            .style(badge::Secondary),
    ]
    .spacing(8)
    .wrap()
    .into()
}
```
