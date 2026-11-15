```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::field::Orientation::Horizontal;
use icedcn::widget::{column, field, select, switch};

#[derive(Clone)]
enum Message {
    Align(bool),
}

fn alignment(aligned: bool) -> Slot<'static, Message> {
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
        .width(Fill)
        .position(if aligned {
            select::Position::ItemAligned
        } else {
            select::Position::Popper
        }),
    ]
    .width(Fill.max(320.0))
    .spacing(20)
    .into()
}
```
