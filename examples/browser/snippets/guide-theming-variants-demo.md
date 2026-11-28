```rust
use iced::Fill;
use icedcn::widget::{badge, button, column, row};
use icedcn::{View, card};

pub fn deployment<'a>() -> View<'a> {
    card(column![
        card::header("api-gateway")
            .description("Build stopped at step 3 of 5.")
            .action(badge("Failed").style(badge::Destructive)),
        card::content(
            row![
                badge("main").style(badge::Outline),
                badge("v2.4.1").style(badge::Secondary),
            ]
            .spacing(8)
        ),
        card::footer(
            row![
                button("Retry"),
                button("View logs").style(button::Outline),
                // Shared variants are also at the crate root: `icedcn::Ghost` is `button::Ghost`.
                button("Dismiss").style(icedcn::Ghost),
            ]
            .spacing(8)
        ),
    ]
    .width(Fill))
    .width(360)
    .into()
}
```
