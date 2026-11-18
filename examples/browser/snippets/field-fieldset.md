```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{field, input, row};

fn address<'a, M: Clone + 'a>(text_size: f32) -> Slot<'a, M> {
    let address = |label, hint| {
        field(input(hint).width(Fill).text_size(text_size))
            .label(label)
            .width(Fill)
    };
    field::set(
        field::legend("Address Information"),
        Some("We need your address to deliver your order."),
        field::group([
            address("Street Address", "123 Main St").into(),
            row![address("City", "New York"), address("Postal Code", "90502")]
                .width(Fill)
                .spacing(16)
                .into(),
        ]),
    )
    .max_width(384.)
    .into()
}
```

The `text_size` parameter lets the caller apply its viewport’s input typography.
