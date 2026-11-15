```rust
use crate::icon;
use iced::alignment::Vertical;
use iced::widget::text::LineHeight;
use iced::{Fill, Padding};
use icedcn::theme;
use icedcn::theme::tweak::{Token, border};
use icedcn::widget::{button, collapsible, column, container, row, space, text};
use icedcn::{Slot, radius};

fn bordered<Message: 'static>(
    content: impl Into<Slot<'static, Message>>,
) -> Slot<'static, Message> {
    container(content)
        .width(Fill)
        .padding([9, 17])
        .class(
            theme::container::Transparent
                + border::Width(1.0)
                + border::Color(Token::Border)
                + radius(radius::MD),
        )
        .into()
}

fn detail<Message: 'static>(title: &'static str, body: &'static str) -> Slot<'static, Message> {
    bordered(column![
        text::small(title)
            .medium()
            .line_height(LineHeight::Absolute(20.into())),
        text::muted(body).line_height(LineHeight::Absolute(20.into())),
    ])
}

fn demo<Message: Clone + 'static>() -> Slot<'static, Message> {
    collapsible(|c| {
        column![
            column![
                container(
                    row![
                        text::small("Order #4189")
                            .semibold()
                            .line_height(LineHeight::Absolute(20.into())),
                        space::horizontal(),
                        c.trigger(
                            button(icon::chevrons_up_down().size(16).line_height(1.0))
                                .square()
                                .style(button::Ghost)
                        ),
                    ]
                    .spacing(16)
                    .align_y(Vertical::Center)
                    .width(Fill)
                )
                .padding([0, 16]),
                bordered(
                    row![
                        text::muted("Status").line_height(LineHeight::Absolute(20.into())),
                        space::horizontal(),
                        text::small("Shipped")
                            .medium()
                            .line_height(LineHeight::Absolute(20.into())),
                    ]
                    .width(Fill)
                ),
            ]
            .spacing(8)
            .width(Fill),
            c.content(
                column![
                    detail("Shipping address", "100 Market St, San Francisco"),
                    detail("Items", "2x Studio Headphones"),
                ]
                .spacing(8)
                .width(Fill)
                .padding(Padding::ZERO.top(8))
            ),
        ]
        .width(Fill)
        .into()
    })
    .width(Fill.max(350))
    .into()
}

fn view<'a>() -> Slot<'a, ()> {
    demo()
}
```
