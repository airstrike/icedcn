use super::icons;
use crate::Message;
use icedcn::Size::*;
use icedcn::Slot;
use icedcn::widget::{Row, button, column, row};

pub fn view<'a>(wide: bool) -> Slot<'a, Message> {
    let pairs: [Slot<'a, Message>; 4] = [
        row![
            button("Extra Small").style(button::Outline).size(Xs),
            button(icons::arrow_up_right(12.0))
                .square()
                .style(button::Outline)
                .size(Xs),
        ]
        .spacing(8)
        .into(),
        row![
            button("Small").style(button::Outline).size(Sm),
            button(icons::arrow_up_right(16.0))
                .square()
                .style(button::Outline)
                .size(Sm),
        ]
        .spacing(8)
        .into(),
        row![
            button("Default").style(button::Outline),
            button(icons::arrow_up_right(16.0))
                .square()
                .style(button::Outline),
        ]
        .spacing(8)
        .into(),
        row![
            button("Large").style(button::Outline).size(Lg),
            button(icons::arrow_up_right(16.0))
                .square()
                .style(button::Outline)
                .size(Lg),
        ]
        .spacing(8)
        .into(),
    ];
    if wide {
        Row::with_children(pairs)
            .spacing(32)
            .align_y(iced::Top)
            .into()
    } else {
        column(pairs).spacing(32).into()
    }
}
