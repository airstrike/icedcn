```rust
use icedxdy::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Payment Method").style(button::Outline)),
    move |_| {
        menu::group([menu::group([
            menu::label("Select Payment Method"),
            menu::group([
                menu::radio_item(
                    with_icon(icon::credit_card(), "Credit Card"),
                    "card",
                    Some(state.payment),
                    Choice::Payment,
                )
                .into(),
                menu::radio_item(
                    with_icon(icon::wallet(), "PayPal"),
                    "paypal",
                    Some(state.payment),
                    Choice::Payment,
                )
                .into(),
                menu::radio_item(
                    with_icon(icon::building_2(), "Bank Transfer"),
                    "bank",
                    Some(state.payment),
                    Choice::Payment,
                )
                .into(),
            ]),
        ])])
    },
)
.width(224)
```
