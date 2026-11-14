## Usage

```rust
use iced::{Element, Fill};
use icedcn::{card, Theme};
use icedcn::widget::{column, text};

fn preview() -> Element<'static, (), Theme> {
    card(column![
        card::header("Card Title")
            .description("Card Description")
            .action(card::action(text::small("Card Action"))),
        card::content(text::small("Card Content")),
        card::footer(text::small("Card Footer")),
    ].width(Fill))
    .width(Fill.max(384))
    .into()
}
```
