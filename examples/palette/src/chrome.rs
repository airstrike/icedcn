//! Standalone palette header; the browser supplies its own navigation.

use iced::widget::space;
use iced::{Center, Fill};
use icedcn::Theme;
use icedcn::widget::{Size, button, container, row, text};

use crate::{Message, Mode, State, icon};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let mode = if state.mode == Mode::Light {
        Mode::Dark
    } else {
        Mode::Light
    };
    container(
        row![
            text::small("icedcn / Create").semibold(),
            space::horizontal(),
            button(icon::sun().size(16))
                .style(button::Ghost)
                .size(Size::Sm)
                .on_press(Message::SetMode(mode)),
            button("Get Code")
                .size(Size::Sm)
                .on_press(Message::CodeOpen(true)),
        ]
        .spacing(12)
        .align_y(Center)
        .width(Fill),
    )
    .padding([12, 24])
    .width(Fill)
    .into()
}
