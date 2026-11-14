```rust
use iced::alignment::Vertical;
use iced::widget::{svg, text::LineHeight};
use iced::{Fill, Padding};
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{button, collapsible, column, container, row, space, text};
use icedxdy::{Slot, Theme, radius};

fn icon(path: &'static str, size: f32) -> svg::Svg<'static, Theme> {
    svg(svg::Handle::from_memory(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    ).into_bytes()))
    .width(size).height(size)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}


fn bordered<Message: 'static>(
    content: impl Into<Slot<'static, Message>>,
) -> Slot<'static, Message> {
    container(content)
        .width(Fill)
        .padding([9, 17])
        .style(|theme: &Theme| Style {
            border: Border {
                width: 1.0,
                color: theme.neutral.border,
                radius: radius::MD.into(),
            },
            ..Style::default()
        })
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
                            button(icon(r#"<path d="m7 15 5 5 5-5M7 9l5-5 5 5"/>"#, 16.))
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


fn view() -> Slot<'static, ()> {
    demo()
}
```
