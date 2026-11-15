//! Stock performance area chart using illustrative prices.

use iced::{Fill, Padding};
use icedcn::widget::chart::Data;
use icedcn::widget::chart::hyozu::{area, areas};
use icedcn::widget::{badge, chart, column, row, text};
use icedcn::{Theme, card};

use crate::{Message, Mode, State, customize};

/// Illustrative daily closing prices, independent of live market data.
pub fn data(color: &str, mode: Mode) -> Data {
    let color: iced::Color = customize::accent(color, mode == Mode::Dark).into();
    chart::data(areas([area([
        183.2, 184.6, 183.9, 187.3, 186.8, 189.2, 191.4,
    ])
    .with_name("AAPL")
    .color(color)
    .opacity(0.18)
    .stroke(2.0)]))
    .x_axis_labels(["May 20", "21", "22", "23", "24", "28", "29"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(true))
    .y_axis_bounds(180.0, 195.0)
    .value_format(|value| format!("${value:.2}"))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column![
            card::header("Stock Performance").description("AAPL · Sample daily closing prices"),
            card::content(
                column![
                    row![text::h2("$191.40"), badge("+4.5%").style(badge::Secondary)].spacing(12),
                    chart(&state.stocks)
                        .height(180)
                        .padding(0)
                        .style(chart::transparent),
                ]
                .spacing(16)
            ),
            card::footer(text::muted("+$8.20 over the displayed period")),
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
