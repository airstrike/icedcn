```rust
use crate::icon;
use iced::Fill;
use iced::widget::text::LineHeight;
use icedcn::theme;
use icedcn::theme::tweak::{Bg, Token};
use icedcn::widget::{button, collapsible, column, container, space, text};
use icedcn::{Size, Slot, card, radius};

fn basic<Message: Clone + 'static>() -> Slot<'static, Message> {
    card(
        card::content(
            collapsible(|c| {
                let open = c.is_open();
                let indicator = if open { icon::chevron_up() } else { icon::chevron_down() };
                let surface = theme::container::Transparent + radius(radius::MD);
                let surface = if open { surface + Bg(Token::Muted) } else { surface };
                container(
                    column![
                        c.trigger(
                            button!["Product details", space::horizontal(), indicator.size(16).line_height(1.0)]
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
                .class(surface)
                .into()
            })
            .width(Fill),
        )
        .padding([16, 16]),
    ).gap(0).padding(0)
    .width(Fill.max(384))
    .into()
}

fn view<'a>() -> Slot<'a, ()> {
    basic()
}
```
