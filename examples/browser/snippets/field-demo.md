```rust
use iced::Fill;
use icedcn::Horizontal;
use icedcn::Slot;
use icedcn::widget::{button, checkbox, column, field, row, select, separator, text, textarea};

fn payment<'a, M: Clone + 'static>(
    required: impl Fn(&'a str, &'a str) -> field::Field<'a, M>,
    submit: M,
) -> Slot<'a, M> {
    let date = |range: std::ops::RangeInclusive<u16>, hint| {
        select(range.map(|n| {
            let label = format!("{n:02}");
            select::item(label.clone(), label)
        }))
        .placeholder(hint)
        .width(Fill)
    };
    column![
        field::set(
            field::legend("Payment Method"),
            Some("All transactions are secure and encrypted"),
            field::group([
                required("Name on Card", "Evil Rabbit").into(),
                required("Card Number", "1234 5678 9012 3456")
                    .description("Enter your 16-digit card number")
                    .into(),
                row![
                    field(date(1..=12, "MM")).label("Month").width(Fill),
                    field(date(2024..=2029, "YYYY")).label("Year").width(Fill),
                    required("CVV", "123").width(Fill)
                ]
                .width(Fill)
                .spacing(16)
                .into(),
            ])
        ),
        separator(),
        field::set(
            field::legend("Billing Address"),
            Some("The billing address associated with your payment method"),
            field(checkbox().default_checked(true))
                .orientation(Horizontal)
                .label(text::small("Same as shipping address"))
        ),
        field(textarea("Add any additional comments")).label("Comments"),
        row![
            button("Submit").on_press(submit),
            button("Cancel").style(button::Outline)
        ]
        .spacing(8),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(448.)
    .into()
}
```

Pass your required-field constructor and submit message to `payment`. The Form section shows how to connect the constructor to your application’s values.
