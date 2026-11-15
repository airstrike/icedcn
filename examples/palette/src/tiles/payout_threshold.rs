//! Preview 01 payout-threshold form card.

use iced::widget::space;
use iced::{Bottom, Fill, Padding};
use icedcn::card;
use icedcn::core::Border;
use icedcn::widget::{button, column, field, row, slider, text, textarea};
use icedcn::{Size, Space, Theme, font_size, radius};

use super::select;
use crate::{Message, State, icon};

const CURRENCIES: &[&str] = &[
    "USD — United States Dollar",
    "EUR — Euro",
    "GBP — British Pound",
    "JPY — Japanese Yen",
];
pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let header = card::header("Payout Threshold")
        .description("Set the minimum balance required before a payout is triggered.")
        .action(
            button(icon::x().size(16))
                .style(button::Ghost)
                .size(Size::Sm)
                .square()
                .style(close_button_style())
                .on_press(Message::Noop),
        );

    let amount = row![
        text::small("Minimum Payout Amount").medium(),
        space::horizontal(),
        text::p(format!("${:.2}", state.payout_threshold))
            .size(font_size::XL2)
            .semibold(),
    ]
    .align_y(Bottom)
    .width(Fill);

    let bounds = row![
        text::muted("$50 (MIN)"),
        space::horizontal(),
        text::muted("$10,000 (MAX)"),
    ]
    .width(Fill);

    let body = field::group([
        field::field(select::trigger("USD — United States Dollar", CURRENCIES))
            .label("Preferred Currency")
            .into(),
        field::field(
            column![
                amount,
                slider(50.0..=10000.0)
                    .value(state.payout_threshold)
                    .on_value_change(Message::SetPayoutThreshold)
                    .width(Fill),
                bounds,
            ]
            .spacing(Space::U2)
            .width(Fill),
        )
        .into(),
        field::field(textarea("Add a note about your payout preferences.").min_height(90.0))
            .label("Notes")
            .into(),
    ]);

    card::card(
        column![
            header,
            card::content(body),
            card::footer(button("Save Threshold").width(Fill).on_press(Message::Noop)),
        ]
        .spacing(Space::U4)
        .padding(Padding::ZERO.top(Space::U4))
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}

fn close_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, _status| iced_widget::button::Style {
        background: Some(theme.muted.base.into()),
        text_color: theme.muted.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        }
        .into(),
        ..Default::default()
    })
}
