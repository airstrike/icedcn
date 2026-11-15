//! Preferences card.

use iced::widget::space;
use iced::{Center, Fill};
use icedcn::widget::{button, column, field, row, switch};
use icedcn::{Size, Theme, card};

use super::select;
use crate::{Message, icon};

const CURRENCIES: &[&str] = &[
    "USD — United States Dollar",
    "EUR — Euro",
    "GBP — British Pound",
    "JPY — Japanese Yen",
];

pub fn view() -> iced::Element<'static, Message, Theme> {
    let currency = field::field(select::trigger("USD — United States Dollar", CURRENCIES))
        .label("Default Currency")
        .width(Fill);

    let public = field::field(switch().default_checked(true))
        .orientation(field::Horizontal)
        .label("Public Statistics")
        .description("Allow others to see your total stream count and listening activity");

    let email = field::field(switch().default_checked(true))
        .orientation(field::Horizontal)
        .label("Email Notifications")
        .description("Monthly royalty reports and distribution updates");

    card::card(
        column![
            card::header("Preferences")
                .description("Manage your account settings and notifications.")
                .action(
                    button(icon::x().size(16))
                        .style(button::Ghost)
                        .size(Size::Sm)
                        .square()
                        .on_press(Message::Noop)
                ),
            card::content(column![currency, public, email].spacing(16).width(Fill)),
            card::footer(
                row![
                    button("Reset")
                        .style(button::Outline)
                        .on_press(Message::Noop),
                    space::horizontal(),
                    button("Save Preferences").on_press(Message::Noop),
                ]
                .align_y(Center)
                .width(Fill)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}
