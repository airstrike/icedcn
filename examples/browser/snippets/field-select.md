```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{field, select};

fn department<M: Clone + 'static>() -> Slot<'static, M> {
    let options = [
        ("engineering", "Engineering"),
        ("design", "Design"),
        ("marketing", "Marketing"),
        ("sales", "Sales"),
        ("support", "Customer Support"),
        ("hr", "Human Resources"),
        ("finance", "Finance"),
        ("operations", "Operations"),
    ];
    field(
        select(options.map(|(id, name)| select::item(id, name)))
            .placeholder("Choose department")
            .width(Fill),
    )
    .label("Department")
    .description("Select your department or area of work.")
    .width(320)
    .into()
}
```
