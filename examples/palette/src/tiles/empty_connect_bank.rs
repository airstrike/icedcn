//! Empty connect bank card.

use iced::Fill;
use iced::Widget;
use icedcn::Theme;
use icedcn::card;
use icedcn::widget::{button, column, empty};

use crate::{Message, State, icon};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media(icon::square().size(16)).style(empty::Icon))
        .title("Connect Bank")
        .description(
            "Link your payout method to receive monthly royalty\n\
             distributions automatically.",
        )
        .content(button("Set Up Payouts").on_press(Message::Noop));

    card::card(column![card::content(content)].padding([16, 0]).width(Fill))
        .width(Fill)
        .boxed()
}
