```rust
use iced::widget;
use iced::{Element, Widget, alignment};
use icedcn::Theme;
use icedcn::widget::badge;

fn example() -> impl Widget<(), Theme> {
    widget::row::<Element<'_, _, Theme>>([
        badge("Badge").into(),
        badge("Secondary").style(badge::Secondary).into(),
        badge("Destructive").style(badge::Destructive).into(),
        badge("Outline").style(badge::Outline).into(),
    ])
    .spacing(8)
    .wrap()
    .align_x(alignment::Horizontal::Center)
}
```
