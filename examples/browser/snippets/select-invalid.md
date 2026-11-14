```rust
use iced::Fill;
use icedcn::theme;
use icedcn::widget::{field, select, text};

field(
    select([select::group([
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
    ])])
    .placeholder("Select a fruit")
    .style(select::Invalid)
    .width(Fill),
)
.label(field::label("Fruit").style(theme::text::Destructive))
.error(text::error("Please select a fruit.").line_height(20. / 14.))
.width(192)
```
