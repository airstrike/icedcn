use crate::{Message, icon};
use icedcn::widget::{Row, button, column, row};
use icedcn::{Size, View};

pub fn view(wide: bool) -> View<'static, Message> {
    let pairs = [
        ("Extra Small", Size::Xs, 12),
        ("Small", Size::Sm, 16),
        ("Default", Size::Default, 16),
        ("Large", Size::Lg, 16),
    ]
    .map(|(label, size, pixels)| {
        row![
            button(label).size(size).style(button::Outline),
            button(icon::arrow_up_right().size(pixels))
                .size(size)
                .square()
                .style(button::Outline),
        ]
        .spacing(8)
        .into()
    });

    if wide {
        Row::with_children(pairs)
            .spacing(32)
            .align_y(iced::Top)
            .into()
    } else {
        column(pairs).spacing(32).into()
    }
}
