//! Weekly power usage with daily consumption bars.

use iced::{Fill, Padding};
use icedcn::widget::chart::{Data, bar, bars};
use icedcn::widget::{chart, column, row, text};
use icedcn::{Theme, card};

use crate::{Message, Mode, State, customize};

/// Daily consumption in kilowatt-hours.
pub fn data(color: &str, mode: Mode) -> Data {
    let color: iced::Color = customize::accent(color, mode == Mode::Dark).into();
    chart::data(
        bars([bar([12.4, 10.8, 14.2, 9.6, 11.3, 8.9, 13.5])
            .with_name("Usage")
            .with_color(color)])
        .data_labels(None),
    )
    .x_axis_labels(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(true))
    .value_format(|value| format!("{value:.1} kWh"))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column![
            card::header("Power Usage").description("Daily consumption · this week"),
            card::content(
                column![
                    row![text::h2("80.7"), text::muted("kWh this week")].spacing(8),
                    chart(&state.power)
                        .height(170)
                        .padding(0)
                        .style(chart::transparent),
                ]
                .spacing(16)
            ),
            card::footer(text::muted("11.5 kWh daily average · Peak on Wednesday")),
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
