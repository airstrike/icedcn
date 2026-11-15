//! Quarterly dividend income with a monthly breakdown.

use std::borrow::Cow;

use iced::{Fill, Padding};
use icedcn::widget::chart::{Data, bar, bars};
use icedcn::widget::{chart, column, text};
use icedcn::{Theme, card};

use crate::{Message, State};

/// Monthly payouts for the quarter.
pub fn data() -> Data {
    chart::data(bars([bar([245, 312, 386]).with_name("Dividends")]).data_labels(None))
        .x_axis_labels(["Apr", "May", "Jun"])
        .x_axis(|axis| axis.show_line(false).show_grid(false))
        .y_axis(|axis| axis.none().show_grid(true))
        .value_format(|value| format!("${value:.2}"))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column![
            card::header("Q2 Dividend Income").description("Cash dividends received · April–June"),
            card::content(
                column![
                    text::h2("$943.00"),
                    chart(&state.dividends)
                        .design(Cow::<Theme>::Owned(state.theme()))
                        .height(170)
                        .padding(0)
                        .style(chart::transparent),
                ]
                .spacing(16)
            ),
            card::footer(text::muted(
                "$314.33 monthly average · Automatically reinvested"
            )),
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
