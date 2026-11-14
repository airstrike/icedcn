```rust
use iced::widget::{svg, text::LineHeight};
use iced::Fill;
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::{button, collapsible, column, container, space, text};
use icedcn::{Size, Slot, Theme, card, radius};

fn icon(path: &'static str, size: f32) -> svg::Svg<'static, Theme> {
    svg(svg::Handle::from_memory(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    ).into_bytes()))
    .width(size).height(size)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}


fn basic<Message: Clone + 'static>() -> Slot<'static, Message> {
    card(
        card::content(
            collapsible(|c| {
                let open = c.is_open();
                let path = if open {
                    r#"<path d="m18 15-6-6-6 6"/>"#
                } else {
                    r#"<path d="m6 9 6 6 6-6"/>"#
                };
                container(
                    column![
                        c.trigger(
                            button!["Product details", space::horizontal(), icon(path, 16.)]
                                .width(Fill)
                                .style(button::Ghost)
                        ),
                        c.content(
                            column![
                                text::small("This panel can be expanded or collapsed to reveal additional content.")
                                    .line_height(LineHeight::Absolute(20.into()))
                                    .width(Fill),
                                button("Learn More").size(Size::Xs),
                            ]
                            .width(Fill)
                            .spacing(8)
                            .padding([0, 10, 10, 10])
                        ),
                    ]
                    .width(Fill),
                )
                .width(Fill)
                .style(move |theme: &Theme| Style {
                    background: open.then(|| theme.muted.base.into()),
                    border: Border {
                        radius: radius::MD.into(),
                        ..Border::default()
                    },
                    ..Style::default()
                })
                .into()
            })
            .width(Fill),
        )
        .padding([16, 16]),
    ).gap(0).padding(0)
    .width(Fill.max(384))
    .into()
}


fn view() -> Slot<'static, ()> {
    basic()
}
```
