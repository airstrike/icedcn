//! Selectable sample cover designs; no file upload is implied.

use iced::{Center, Fill};
use icedcn::theme::container::card_muted;
use icedcn::widget::{column, container, tabs, text};
use icedcn::{Theme, card};

use crate::{Message, State};

pub fn view(_: &State) -> iced::Element<'static, Message, Theme> {
    card(
        column![
            card::header("Cover Art").description("Choose a sample cover for your next release."),
            card::content(
                tabs(|t| {
                    let covers = [
                        ("orbit", "Orbit", "◯", "LUNA PARK"),
                        ("waves", "Waves", "≈", "THE SUNDAY CLUB"),
                        ("dawn", "Dawn", "☼", "SOFT FOCUS"),
                    ];
                    let previews = covers.map(|(id, title, mark, artist)| {
                        t.content(
                            id,
                            container(
                                column![
                                    text::large(mark).size(72),
                                    text::large(title).semibold(),
                                    text::xs(artist),
                                ]
                                .spacing(6)
                                .align_x(Center),
                            )
                            .center_x(Fill)
                            .center_y(200)
                            .style(card_muted),
                        )
                    });
                    column![
                        t.list(covers.map(|(id, title, _, _)| t.trigger(id, title))),
                        column(previews).width(Fill),
                        text::muted("Sample artwork · preview only"),
                    ]
                    .spacing(12)
                    .width(Fill)
                    .into()
                })
                .default_value("orbit")
                .width(Fill)
                .list_width(Fill)
            ),
        ]
        .spacing(16),
    )
    .width(Fill)
    .into()
}
