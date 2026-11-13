```rust
use iced::Widget;
use iced::widget::responsive;
use icedxdy::Size::*;
use icedxdy::widget::{Row, button, column, container, row};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    let layout = responsive(|bounds| -> iced::Element<'a, Message, Theme> {
        let pairs: [Slot<'a, Message>; 4] = [
            row![
                button("Extra Small").style(button::Outline).size(Xs),
                button(icon::arrow_up_right())
                    .square()
                    .style(button::Outline)
                    .size(Xs),
            ]
            .spacing(8)
            .into(),
            row![
                button("Small").style(button::Outline).size(Sm),
                button(icon::arrow_up_right())
                    .square()
                    .style(button::Outline)
                    .size(Sm),
            ]
            .spacing(8)
            .into(),
            row![
                button("Default").style(button::Outline),
                button(icon::arrow_up_right())
                    .square()
                    .style(button::Outline),
            ]
            .spacing(8)
            .into(),
            row![
                button("Large").style(button::Outline).size(Lg),
                button(icon::arrow_up_right())
                    .square()
                    .style(button::Outline)
                    .size(Lg),
            ]
            .spacing(8)
            .into(),
        ];
        let content: Slot<'a, Message> = if bounds.width < 520.0 {
            column(pairs).spacing(32).into()
        } else {
            Row::with_children(pairs).spacing(32).into()
        };
        container(content).center(iced::Fill).into()
    });
    let layout: iced::Element<'a, Message, Theme> = layout.boxed();
    layout.into()
}

```
