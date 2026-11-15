//! Monthly traffic channels rendered by Hyozu.

use iced::Fill;
use iced::widget::space;
use icedcn::Theme;
use icedcn::card;
use icedcn::theme;
use icedcn::widget::chart::{Data, bar, bars, legend};
use icedcn::widget::{badge, chart, column, container, row, text};

use crate::{Message, State};

pub fn data() -> Data {
    chart::data(
        bars([
            bar([186, 305, 237, 73, 209, 214]).with_name("Desktop"),
            bar([80, 200, 120, 190, 130, 140]).with_name("Mobile"),
        ])
        .data_labels(None),
    )
    .x_axis_labels(["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(true))
    .legend(legend::Config::below().size(12).interactive(true))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let footer = row![
        text::large("2,084"),
        text::xs("visits").style(theme::text::Muted),
        space::horizontal(),
        badge("Jan–Jun").style(badge::Secondary),
    ]
    .spacing(8)
    .width(Fill);

    let body = column![chart(&state.traffic).height(156).padding(0), footer]
        .spacing(16)
        .width(Fill);

    card(
        column![
            card::header("Traffic channels").description(
                "Monthly desktop and mobile visitors. Select a legend to compare channels."
            ),
            card::content(container(body).padding([0, 16]).width(Fill)),
        ]
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .height(412.0)
    .into()
}
