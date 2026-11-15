//! Savings progress with a readable goal breakdown.

use iced::{Fill, Padding};
use icedcn::widget::{column, progress, row, text};
use icedcn::{Theme, card};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    card::card(
        column![
            card::header("Savings Progress").description("Building your emergency fund"),
            card::content(
                column![
                    row![text::h2("$12,450"), text::muted("of $20,000")].spacing(8),
                    progress(62.25).width(Fill),
                    text::small("62% of your goal reached"),
                    text::muted("$7,550 remaining · Saving $500 each month"),
                ]
                .spacing(16)
            ),
            card::footer(text::muted("On track to reach your goal in 16 months")),
        ]
        .spacing(16)
        .padding(Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}
