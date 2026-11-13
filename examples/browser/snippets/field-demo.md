```rust
pub(super) fn checkout<'a>() -> Element<'a, Message, Theme> {
    let month = select((1..=12).map(|n| { let m = format!("{n:02}"); select::item(m.clone(), m) })).placeholder("MM").width(Fill);
    let year = select((2024..=2029).map(|n| select::item(n.to_string(), n.to_string()))).placeholder("YYYY").width(Fill);
    column![
        field::set(field::legend("Payment Method"), Some("All transactions are secure and encrypted"), field::group([
            field(input("Evil Rabbit")).label("Name on Card").into(),
            field(input("1234 5678 9012 3456")).label("Card Number").description("Enter your 16-digit card number").into(),
            row![field(month).label("Month").width(Fill), field(year).label("Year").width(Fill),
                 field(input("123")).label("CVV").width(Fill)].spacing(16).into(),
        ])),
        separator(),
        field::set(field::legend("Billing Address"), Some("The billing address associated with your payment method"),
            field(checkbox().default_checked(true)).orientation(Horizontal).label(text::small("Same as shipping address"))),
        field(textarea("Add any additional comments")).label("Comments"),
        row![button("Submit"), button("Cancel").style(button::Outline)].spacing(8)
    ].spacing(20).width(Fill).max_width(448.).into()
}
```
