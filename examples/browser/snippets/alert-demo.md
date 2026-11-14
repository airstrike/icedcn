```rust
use iced::widget::svg;
use iced::Widget;
use icedxdy::Theme;
use iced::Fill;
use icedxdy::widget::{alert, column};

    column![
        alert()
            .icon(
svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#.as_bytes()))
                .width(16)
                .height(16)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.card.foreground.into()) })
                .boxed(),
)
            .title("Payment successful")
            .description(
                "Your payment of $29.99 has been processed. A receipt has been sent to your email address.",
            )
            .max_width(448),
        alert()
            .icon(
svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#.as_bytes()))
                .width(16)
                .height(16)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.card.foreground.into()) })
                .boxed(),
)
            .title("New feature available")
            .description(
                "We've added dark mode support. You can enable it in your account settings.",
            )
            .max_width(448),
    ]
    .spacing(16)
    .width(Fill.max(448))
```
