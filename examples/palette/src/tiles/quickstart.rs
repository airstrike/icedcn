//! Overview tile — exercises rich text, an aspect-ratio thumbnail,
//! a beta badge, a deploy button with a spinner state, and the blockquote
//! text helper.
//!
//! Exercises: rich_text, aspect_ratio, badge, spinner, button (primary),
//! text variants (large, p, muted, blockquote), container, row, column.

use iced::Fill;
use icedcn::Theme;
use icedcn::View;
use icedcn::card;
use icedcn::theme::container::card_muted;
use icedcn::theme::tweak::Token;
use icedcn::widget::{
    aspect_ratio, badge, button, column, container, rich_text, row, space, spinner, text,
};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let title_row = row![
        text::large("Overview"),
        space::horizontal(),
        badge("Beta").style(badge::Secondary),
    ]
    .spacing(8)
    .width(Fill);

    // rich_text — body emphasises the "Deploy" CTA with a token color
    // that re-resolves on theme flip.
    let teaser = rich_text![
        rich_text::span("Status: ").size(text::size::SM),
        rich_text::span("primary CTA wired to ")
            .size(text::size::SM)
            .color(Token::MutedForeground),
        rich_text::span("Deploy")
            .size(text::size::SM)
            .medium()
            .color(Token::Primary),
        rich_text::span(". Flip the header switch to disable every action at once.")
            .size(text::size::SM)
            .color(Token::MutedForeground),
    ];

    let deploy_label: View<'_, Message> = if state.deploying {
        row![spinner(), "Deploying"].spacing(8).into()
    } else {
        "Deploy".into()
    };
    let mut deploy = button(deploy_label).width(160.0);
    if state.connected {
        deploy = deploy.on_press(Message::Deploy);
    }

    // Reserve a 16:9 area for the preview.
    let banner = aspect_ratio(
        container(text::muted("preview"))
            .center_x(Fill)
            .center_y(Fill)
            .style(card_muted),
        16.0 / 9.0,
    )
    .max_width(280.0);

    let quote = text::blockquote("Components you control, with the iced rendering pipeline.");

    let body = column([
        title_row.into(),
        View::from(teaser),
        View::from(banner),
        quote.into(),
        deploy.into(),
    ])
    .spacing(12);

    card::card(
        column([
            card::header("Welcome to icedcn")
                .description("Themed widgets for iced applications.")
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}
