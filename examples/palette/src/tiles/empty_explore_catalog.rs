//! Empty explore catalog card.

use iced::Fill;
use icedcn::Theme;
use icedcn::card;
use icedcn::widget::{button, column, empty};

use crate::{Message, State, icon};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media(icon::circle().size(16)).style(empty::Icon))
        .title("Explore Catalog")
        .description(
            "Check your ISRC codes, metadata, and visual assets\n\
             before going live.",
        )
        .content(button("View Catalog").on_press(Message::Noop));

    card::card(column![card::content(content)].padding([16, 0]).width(Fill))
        .gap(0)
        .padding(0)
        .width(Fill)
        .into()
}
