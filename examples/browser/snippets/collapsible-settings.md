```rust
use iced::{Fill, Widget};
use icedxdy::{card, Slot};
use icedxdy::widget::{button, collapsible, column, input, row};

card(
    column![
        card::header("Radius").description("Set the corner radius of the element."),
        Slot::from(
            card::content(
                collapsible(|c| {
                    let inputs = || {
                        row![
                            input("0").default_value("0").width(Fill),
                            input("0").default_value("0").width(Fill)
                        ]
                        .spacing(8)
                        .width(Fill)
                    };
                    let icon = if c.is_open() {
                        icon::minimize()
                    } else {
                        icon::maximize()
                    };
                    row![
                        column![inputs(), c.content(inputs())]
                            .spacing(8)
                            .width(Fill),
                        c.trigger(button(icon).square().style(button::Outline)),
                    ]
                    .spacing(8)
                    .width(Fill)
                    .into()
                })
                .width(Fill)
            )
            .boxed()
        ),
    ]
    .spacing(12)
    .padding([12, 0]),
)
.width(Fill.max(320.0))
.boxed()
```
