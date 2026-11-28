use crate::Message;
use iced::widget::svg;
use icedcn::View;
use icedcn::widget::button;
use icedcn::{Size, Theme};

pub fn view() -> View<'static, Message> {
    let branch = svg(svg::Handle::from_memory(
        include_bytes!("../../../assets/button/git-branch.svg").as_slice(),
    ))
    .width(14)
    .height(14)
    .style(|theme: &Theme, _| svg::Style {
        color: Some(theme.neutral.foreground.into()),
    });

    button(branch)
        .push("New Branch")
        .size(Size::Sm)
        .style(button::Outline)
        .into()
}
