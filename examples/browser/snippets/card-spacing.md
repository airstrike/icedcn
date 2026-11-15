Pass the login card from the Demo and keep the selected spacing in your application model, starting at 16.

```rust
use iced::Fill;
use icedcn::widget::{column, text, toggle_group};
use icedcn::{Size, Slot, card::Card};

#[derive(Clone)]
enum Message {
    Spacing(u16),
}

fn spacing_view(spacing: u16, login: Card<'_, Message>) -> Slot<'_, Message> {
    let choices = toggle_group(|| {
        [16, 20, 24, 32].map(|n| toggle_group::item(n.to_string(), text::small(format!("{n}px"))))
    })
    .style(toggle_group::Outline)
    .size(Size::Sm)
    .spacing(8)
    .value([spacing.to_string()])
    .on_value_change(move |selected| {
        let value = selected
            .first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(spacing);
        Message::Spacing(value)
    });

    column![choices, login.spacing(f32::from(spacing))]
        .spacing(16)
        .width(Fill.max(384))
        .into()
}
```
