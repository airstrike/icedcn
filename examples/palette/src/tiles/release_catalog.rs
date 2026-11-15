//! Searchable sample release catalog.

use iced::widget::component::{Component, component};
use iced::{Center, Element, Fill, Widget};
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{avatar, badge, column, input, input_group, row, separator, space, text};
use icedcn::{Theme, card};

use crate::{Message, icon};

const RELEASES: &[(&str, &str, &str)] = &[
    ("Midnight City", "Luna Park · Single", "Live"),
    ("Soft Focus", "Luna Park · EP", "Scheduled"),
    ("After Hours", "The Sunday Club · Album", "Draft"),
];

struct Preview;

impl<'a> Component<'a, Message, Theme> for Preview {
    type State = String;
    type Event = String;

    fn update(&self, query: &mut String, value: String, _: &iced::Renderer) -> Option<Message> {
        *query = value;
        None
    }

    fn view(&self, query: &String) -> impl Widget<String, Theme> + 'a {
        let query_lower = query.to_lowercase();
        let releases: Vec<_> = RELEASES
            .iter()
            .filter(|(title, artist, _)| {
                title.to_lowercase().contains(&query_lower)
                    || artist.to_lowercase().contains(&query_lower)
            })
            .collect();
        let search = input_group(
            input("Search releases or artists...")
                .value(query.clone())
                .on_input(|value| value),
        )
        .addon(addon(icon::search().size(16)))
        .addon(addon(text::muted(format!("{} releases", releases.len()))).align(Align::InlineEnd));
        let mut rows = column![].spacing(12).width(Fill);
        for (title, artist, status) in &releases {
            rows = rows
                .push(
                    row![
                        avatar(None).fallback("LP"),
                        column![text::small(*title).semibold(), text::muted(*artist)].spacing(2),
                        space::horizontal(),
                        badge(*status).style(badge::Secondary),
                    ]
                    .spacing(12)
                    .align_y(Center)
                    .into(),
                )
                .push(separator().into());
        }
        if releases.is_empty() {
            rows = rows.push(text::muted("No releases match your search.").into());
        }
        Element::from(
            card(
                column![
                    card::header("Release Catalog")
                        .description("Browse your releases and their distribution status."),
                    card::content(column![search, rows].spacing(16)),
                ]
                .spacing(16),
            )
            .width(Fill),
        )
    }
}

pub fn view(_: &crate::State) -> Element<'static, Message, Theme> {
    component(Preview).boxed()
}
