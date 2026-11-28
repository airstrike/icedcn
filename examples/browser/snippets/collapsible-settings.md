```rust
use crate::icon;
use iced::alignment::Vertical;
use iced::widget::text::LineHeight;
use iced::{Fill, Padding};
use icedcn::widget::{button, collapsible, column, container, input, row, text};
use icedcn::{View, card};

fn settings<'a, Message: Clone + 'static>() -> View<'a, Message> {
    card(
        column![
            card::header(text::small("Radius").medium().line_height(1.375))
                .inset(12.)
                .description(
                    text::muted("Set the corner radius of the element.")
                        .line_height(LineHeight::Absolute(20.into()))
                ),
            card::content(
                collapsible(|c| {
                    let inputs = || {
                        row![
                            input("0").default_value("0").width(Fill),
                            input("0").default_value("0").width(Fill),
                        ]
                        .spacing(8)
                        .width(Fill)
                    };
                    let indicator = if c.is_open() {
                        icon::minimize()
                    } else {
                        icon::maximize()
                    };
                    row![
                        column![
                            inputs(),
                            c.content(
                                container(inputs())
                                    .width(Fill)
                                    .padding(Padding::ZERO.top(8))
                            ),
                        ]
                        .width(Fill),
                        c.trigger(
                            button(indicator.size(16).line_height(1.0))
                                .square()
                                .style(button::Outline)
                        ),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Top)
                    .width(Fill)
                    .into()
                })
                .width(Fill)
            )
            .padding([0, 12]),
        ]
        .spacing(12)
        .padding([12, 0])
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill.max(320))
    .into()
}

fn view<'a>() -> View<'a> {
    settings()
}
```
