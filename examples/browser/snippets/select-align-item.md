```rust
use iced::Fill;
use icedcn::widget::{field, column, switch, select};
use icedcn::widget::field::Orientation::Horizontal;

column![
    field(switch().checked(aligned).on_checked_change(Message::Align))
        .orientation(Horizontal)
        .control_last()
        .label("Align Item")
        .description("Toggle to align the item with the trigger."),
    select([select::group([
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
        select::item("grapes", "Grapes"),
        select::item("pineapple", "Pineapple")
    ])])
    .default_value("banana")
    .icon(icon::chevron_down)
    .position(if aligned {
        select::Position::ItemAligned
    } else {
        select::Position::Popper
    }),
]
.width(Fill.max(320.0))
.spacing(16)
```
