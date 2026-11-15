//! Preview 01 contribution history card.

use std::borrow::Cow;

use iced::{Fill, Padding};
use icedcn::Slot;
use icedcn::Theme;
use icedcn::card;
use icedcn::core::Border;
use icedcn::theme;
use icedcn::theme::container::Style;
use icedcn::widget::chart::{Data, bar, bars, hyozu};
use icedcn::widget::{button, chart, column, container, row, text};

use crate::{Message, PreviewPage, State};

/// Six months of contributions, tinted by the designer's chart selection.
pub fn data() -> Data {
    chart::data(
        bars([bar([1200, 1650, 1350, 1950, 1100, 2100])
            .with_name("Contributions")
            .with_color(hyozu::Color::Secondary)])
        .data_labels(None),
    )
    .x_axis_labels(["Dec", "Jan", "Feb", "Mar", "Apr", "May"])
    .x_axis(|axis| axis.show_line(false).show_grid(false))
    .y_axis(|axis| axis.none().show_grid(false))
    .value_format(|value| format!("${value:.0}"))
}

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let chart = chart(&state.contributions)
        .design(Cow::<Theme>::Owned(state.theme()))
        .height(180)
        .padding(0)
        .style(chart::transparent);

    let details = row![
        detail("UPCOMING", "May 25, 2024", "$1,000 scheduled"),
        detail("AUTO-SAVE PLAN", "Accelerated", "Recurring weekly"),
    ]
    .spacing(12)
    .width(Fill);

    card::card(
        column![
            card::header("Contribution History").description("Last 6 months of activity"),
            card::content(column![chart, details].spacing(16).width(Fill)),
            card::footer(
                button("More chart previews")
                    .width(Fill)
                    .on_press(Message::SetPreviewPage(PreviewPage::Two))
            ),
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

fn detail(
    label: &'static str,
    title: &'static str,
    description: &'static str,
) -> Slot<'static, Message> {
    container(
        column![
            text::xs(label)
                .medium()
                .letter_spacing(0.08)
                .style(theme::text::Muted),
            text::p(title).semibold(),
            text::muted(description),
        ]
        .spacing(6)
        .width(Fill),
    )
    .padding(12)
    .width(Fill)
    .style(detail_style)
    .into()
}

fn detail_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.45).into()),
        border: Border {
            radius: 10.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
