//! Social links card.

use iced::Fill;
use icedcn::widget::{button, column, field, input, row, space};
use icedcn::{Theme, card};

use crate::Message;

pub fn view() -> iced::Element<'static, Message, Theme> {
    card::card(
        column![
            card::header("Social Links"),
            card::content(
                column![
                    link_field("Spotify Artist URL", "spotify.com/artist/3j...2k"),
                    link_field("Instagram Handle", "@julianduryea_music"),
                    link_field("SoundCloud URL", "soundcloud.com/username"),
                    link_field("Website", "https://yoursite.com"),
                ]
                .spacing(16)
                .width(Fill)
            ),
            card::footer(
                row![
                    space::horizontal(),
                    button("Discard")
                        .style(button::Secondary)
                        .on_press(Message::Noop),
                    button("Save Changes").on_press(Message::Noop),
                ]
                .spacing(8)
                .width(Fill)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}

fn link_field<'a>(label: &'static str, value: &'a str) -> icedcn::Slot<'a, Message> {
    field::field(input("").default_value(value).width(Fill))
        .label(label)
        .width(Fill)
        .into()
}
