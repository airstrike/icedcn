## Usage

```rust
use iced::Fill;
use icedcn::card;
use icedcn::widget::{column, text};

fn preview<'a>() -> card::Card<'a, ()> {
    card(
        column![
            card::header("Card Title")
                .description("Card Description")
                .action(card::action(text::small("Card Action"))),
            card::content(text::small("Card Content")),
            card::footer(text::small("Card Footer")),
        ]
        .width(Fill),
    )
    .width(Fill.max(384))
}
```
