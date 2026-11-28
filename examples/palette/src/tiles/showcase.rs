//! Showcase cards for the palette preview.

use iced::Element;

use iced::widget::{space, stack};
use iced::{Center, Fill};
use icedcn::Theme;
use icedcn::card;
use icedcn::core::{Border, Color};
use icedcn::theme;
use icedcn::widget::button::Button;
use icedcn::widget::row::Row;
use icedcn::widget::{
    Size, Skeleton, View, badge, button, checkbox, column, container, input, row, skeleton, slider,
    switch, text, textarea,
};

use crate::{Message, Mode, State, customize, icon};

const CARD_THEME_HEIGHT: f32 = 292.0;
const CARD_TYPE_HEIGHT: f32 = 360.0;
const CARD_ICONS_HEIGHT: f32 = 114.0;
const CARD_CONTROLS_HEIGHT: f32 = 397.0;
const CARD_SKELETON_HEIGHT: f32 = 190.0;

pub fn theme_card(state: &State) -> iced::Element<'static, Message, Theme> {
    let theme = state.theme();
    let chart = customize::accent(state.settings.chart, state.mode == Mode::Dark);
    let swatches = column![
        swatch_row([
            ("background", theme.neutral.base),
            ("foreground", theme.neutral.foreground),
            ("primary", theme.primary.base),
            ("secondary", theme.secondary.base),
            ("muted", theme.muted.base),
            ("accent", theme.accent.base),
        ]),
        swatch_row([
            ("border", theme.neutral.border),
            ("chart", chart),
            ("chart / 50%", chart.scale_alpha(0.5)),
            ("ring", theme.neutral.ring),
            ("card", theme.card.base),
            ("sidebar", theme.sidebar.background),
        ]),
    ]
    .spacing(16)
    .width(Fill);

    let body = column![
        text::p(format!("Nova · {}", state.settings.font.family))
            .size(24.0)
            .line_height(iced::Pixels(30.0))
            .semibold(),
        text::muted(
            "Designers love packing quirky glyphs into\n\
             test phrases. This is a preview of the..."
        )
        .size(16.0),
        swatches,
    ]
    .spacing(12)
    .width(Fill);

    panel(body, CARD_THEME_HEIGHT)
}

pub fn typography_card(state: &State) -> iced::Element<'static, Message, Theme> {
    let body = column![
        text::xs(format!("INHERIT · {}", state.settings.font.family)).style(theme::text::Muted),
        text::p("Designing with rhythm and\nhierarchy.")
            .size(24.0)
            .line_height(iced::Pixels(30.0))
            .semibold(),
        text::muted(
            "A strong body style keeps long-form content\n\
             readable and balances the visual weight of\n\
             headings."
        )
        .size(16.0),
        text::muted(
            "Thoughtful spacing and cadence help paragraphs\n\
             scan quickly without feeling dense."
        )
        .size(16.0),
        space::vertical(),
        button("Share Feedback")
            .style(button::Outline)
            .size(Size::Sm)
            .width(Fill)
            .on_press(Message::Noop),
    ]
    .spacing(14)
    .width(Fill)
    .height(Fill);

    panel(body, CARD_TYPE_HEIGHT)
}

pub fn icon_grid() -> iced::Element<'static, Message, Theme> {
    let first = row![
        icon_button(icon::copy()),
        icon_button(icon::circle_help()),
        icon_button(icon::trash_2()),
        icon_button(icon::upload()),
        icon_button(icon::archive()),
        icon_button(icon::ellipsis()),
        icon_button(icon::refresh_cw()),
        icon_button(icon::plus()),
    ]
    .spacing(10)
    .width(Fill);

    let second = row![
        icon_button(icon::minus()),
        icon_button(icon::arrow_left()),
        icon_button(icon::arrow_right()),
        icon_button(icon::check()),
        icon_button(icon::chevron_down()),
        icon_button(icon::chevron_right()),
        icon_button(icon::search()),
        icon_button(icon::settings()),
    ]
    .spacing(10)
    .width(Fill);

    panel(
        column![first, second].spacing(12).width(Fill),
        CARD_ICONS_HEIGHT,
    )
}

pub fn controls_card(state: &State) -> iced::Element<'_, Message, Theme> {
    let tabs = row![
        button("Button").size(Size::Sm).on_press(Message::Noop),
        button("Secondary")
            .style(button::Secondary)
            .size(Size::Sm)
            .on_press(Message::Noop),
        button("Outline")
            .style(button::Outline)
            .size(Size::Sm)
            .on_press(Message::Noop),
        button("Ghost")
            .style(button::Ghost)
            .size(Size::Sm)
            .on_press(Message::Noop),
    ]
    .spacing(8)
    .align_y(Center)
    .width(Fill);

    let field = container(row![
        text::small("Two-factor authentication"),
        space::horizontal(),
        button("Enable")
            .style(button::Secondary)
            .size(Size::Sm)
            .on_press(Message::Noop),
    ])
    .padding([8, 12])
    .height(50.0)
    .width(Fill)
    .style(control_style);

    let badges = row![
        badge("Badge"),
        badge("Secondary").style(badge::Secondary),
        badge("Outline").style(badge::Outline),
        space::horizontal(),
        checkbox().default_checked(true),
        checkbox(),
        checkbox().default_checked(true),
    ]
    .spacing(10)
    .align_y(Center)
    .width(Fill);

    let footer = row![
        button("Alert Dialog")
            .style(button::Outline)
            .size(Size::Sm)
            .on_press(Message::Noop),
        button(
            row![
                text::small("Button Group"),
                space::horizontal(),
                icon::chevron_up().size(14)
            ]
            .spacing(10)
            .align_y(Center)
            .width(Fill),
        )
        .style(button::Outline)
        .size(Size::Sm)
        .width(143.0)
        .on_press(Message::Noop),
        space::horizontal(),
        switch()
            .checked(state.connected)
            .on_checked_change(Message::ToggleConnection),
    ]
    .spacing(8)
    .align_y(Center)
    .width(Fill);

    let body = column![
        tabs,
        field,
        slider(0.0..=100.0).default_value(45.0).width(Fill),
        name_input(),
        textarea("Message").resizable(false),
        badges,
        footer,
    ]
    .spacing(16)
    .width(Fill);

    panel(body, CARD_CONTROLS_HEIGHT)
}

pub fn skeleton_card() -> iced::Element<'static, Message, Theme> {
    let lines = column![
        row![
            skeleton().width(40.0).height(40.0).radius(999.0),
            column![skeleton_line(118.0, 16.0), skeleton_line(180.0, 12.0),].spacing(12)
        ]
        .spacing(14)
        .align_y(Center),
        skeleton_line(258.0, 14.0),
        skeleton_line(230.0, 14.0),
        row![skeleton_line(96.0, 32.0), skeleton_line(78.0, 32.0)].spacing(10),
    ]
    .spacing(16)
    .width(Fill);

    panel(lines, CARD_SKELETON_HEIGHT)
}

fn swatch_row(items: [(&'static str, Color); 6]) -> View<'static, Message> {
    Row::with_children(items.into_iter().map(|(label, color)| {
        column![
            swatch(color),
            text::xs(label)
                .size(9.0)
                .line_height(iced::Pixels(12.0))
                .style(theme::text::Muted)
        ]
        .spacing(7)
        .width(48.0)
        .into()
    }))
    .spacing(11)
    .width(Fill)
    .into()
}

fn swatch(color: Color) -> View<'static, Message> {
    container(space::vertical())
        .width(48.0)
        .height(48.0)
        .style(move |theme: &Theme| theme::container::Style {
            background: Some(color.into()),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: theme.neutral.border,
            },
            ..Default::default()
        })
        .into()
}

fn icon_button<'a>(icon: impl Into<View<'a, Message>>) -> Button<'a, Message> {
    button(icon)
        .style(button::Outline)
        .size(Size::Sm)
        .square()
        .on_press(Message::Noop)
}

fn panel<'a>(body: impl Into<View<'a, Message>>, height: f32) -> iced::Element<'a, Message, Theme> {
    card::card(container(body.into()).padding(16).width(Fill).height(Fill))
        .gap(0)
        .padding(0)
        .width(Fill)
        .height(height)
        .into()
}

fn skeleton_line(width: f32, height: f32) -> Skeleton<'static, Message> {
    skeleton().width(width).height(height)
}

fn name_input() -> View<'static, Message> {
    let input = input("Name").width(Fill).on_submit(Message::Noop);

    let trailing = container(icon::search().size(16))
        .width(Fill)
        .height(32.0)
        .padding([0, 10])
        .align_x(iced::Right)
        .align_y(Center);

    stack![Element::from(input), Element::from(trailing)]
        .width(Fill)
        .height(32.0)
        .into()
}

fn control_style(theme: &Theme) -> theme::container::Style {
    theme::container::Style {
        background: Some(theme.muted.base.scale_alpha(0.4).into()),
        text_color: Some(theme.neutral.foreground),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: theme.neutral.border,
        },
        ..Default::default()
    }
}
