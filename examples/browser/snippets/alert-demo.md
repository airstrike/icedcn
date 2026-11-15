```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{alert, column, text};

fn demo<M: Clone + 'static>(wrapping: text::Wrapping) -> Slot<'static, M> {
    let notice = |icon, title, description| {
        alert().icon(icon).title(title).description(description).description_wrapping(wrapping)
    };
    column![
        notice(icon::circle_check().size(16).line_height(1.0),
            "Payment successful",
            "Your payment of $29.99 has been processed. A receipt has been sent to your email address."),
        notice(icon::info().size(16).line_height(1.0),
            "New feature available",
            "We've added dark mode support. You can enable it in your account settings."),
    ].width(Fill.max(448)).spacing(16).into()
}
```
