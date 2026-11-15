//! Card spending and available balance.

use iced::{Fill, Padding};
use icedcn::widget::chart::{Data, bar, bars};
use icedcn::widget::{chart, column, progress, row, text};
use icedcn::{Theme, card};

use crate::{Message, Mode, State, customize};

/// Daily card spending for the last seven days.
pub fn data(color: &str, mode: Mode) -> Data {
    let color: iced::Color = customize::accent(color, mode == Mode::Dark).into();
    chart::data(
        bars([bar([128, 220, 165, 310, 196, 145, 240])
            .with_name("Spending")
            .with_color(color)])
        .data_labels(None),
    )
    .x_axis_labels(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(false))
    .value_format(|value| format!("${value:.2}"))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column![
            card::header("Card Overview").description("Everyday card · •••• 4829"),
            card::content(
                column![
                    row![text::h2("$1,404.00"), text::muted("spent this week")].spacing(8),
                    chart(&state.spending)
                        .height(160)
                        .padding(0)
                        .style(chart::transparent),
                    progress(14.04).width(Fill),
                    text::muted("$8,596 available of your $10,000 limit"),
                ]
                .spacing(16)
            ),
            card::footer(text::muted("Next statement closes June 1")),
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
