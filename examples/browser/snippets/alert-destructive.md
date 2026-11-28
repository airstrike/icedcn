```rust
use crate::icon;
use icedcn::View;
use icedcn::widget::alert;

fn payment_failed<'a, M: 'a>() -> View<'a, M> {
    alert()
        .icon(icon::circle_alert().size(16).line_height(1.0))
        .title("Payment failed")
        .description(
            "Your payment could not be processed. Please check your payment method and try again.",
        )
        .style(alert::Destructive)
        .max_width(448)
        .into()
}
```
