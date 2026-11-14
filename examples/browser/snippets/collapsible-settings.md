```rust
use iced::alignment::Vertical;
use iced::widget::{svg, text::LineHeight};
use iced::{Fill, Padding};
use icedcn::widget::{button, collapsible, column, container, input, row, text};
use icedcn::{Slot, Theme, card};

fn icon(path: &'static str, size: f32) -> svg::Svg<'static, Theme> {
    svg(svg::Handle::from_memory(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    ).into_bytes()))
    .width(size).height(size)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}


fn settings<Message: Clone + 'static>() -> Slot<'static, Message> {
    card(column![
        card::header(text::small("Radius").medium().line_height(1.375))
            .inset(12.)
            .description(
                text::muted("Set the corner radius of the element.")
                    .line_height(LineHeight::Absolute(20.into()))
            ),
        card::content(collapsible(|c| {
            let inputs = || row![
                input("0").default_value("0").width(Fill),
                input("0").default_value("0").width(Fill),
            ]
            .spacing(8)
            .width(Fill);
            let path = if c.is_open() {
                r#"<path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3"/>"#
            } else {
                r#"<path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"/>"#
            };
            row![
                column![
                    inputs(),
                    c.content(container(inputs()).width(Fill).padding(Padding::ZERO.top(8))),
                ]
                .width(Fill),
                c.trigger(button(icon(path, 16.)).square().style(button::Outline)),
            ]
            .spacing(8)
            .align_y(Vertical::Top)
            .width(Fill)
            .into()
        }).width(Fill))
        .padding([0, 12]),
    ]
    .spacing(12)
    .padding([12, 0])
    .width(Fill)).gap(0).padding(0)
    .width(Fill.max(320))
    .into()
}


fn view() -> Slot<'static, ()> {
    settings()
}
```
