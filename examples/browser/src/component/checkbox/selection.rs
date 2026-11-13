use iced::widget::component::{Component, component};
use iced::{Element, Length, Widget};
use icedxdy::widget::{checkbox, table, text};
use icedxdy::{Slot, Theme};

use crate::Message;

const USERS: [(&str, &str, &str); 4] = [
    ("Sarah Chen", "sarah.chen@example.com", "Admin"),
    ("Marcus Rodriguez", "marcus.rodriguez@example.com", "User"),
    ("Priya Patel", "priya.patel@example.com", "User"),
    ("David Kim", "david.kim@example.com", "Editor"),
];

pub(super) fn preview<'a>() -> Element<'a, Message, Theme> {
    component(Selection).boxed()
}

struct Selection;

#[derive(Debug, Clone)]
enum Change {
    All(bool),
    Row(usize, bool),
}

struct Rows([bool; USERS.len()]);

impl Default for Rows {
    fn default() -> Self {
        Self([true, false, false, false])
    }
}

impl<'a> Component<'a, Message, Theme> for Selection {
    type State = Rows;
    type Event = Change;

    fn update(&self, state: &mut Rows, change: Change, _: &iced::Renderer) -> Option<Message> {
        match change {
            Change::All(selected) => state.0.fill(selected),
            Change::Row(row, selected) => state.0[row] = selected,
        }
        None
    }

    fn view(&self, state: &Rows) -> impl Widget<Change, Theme> + 'a {
        let header = table::header([table::row([
            table::head(
                checkbox()
                    .checked(state.0.iter().all(|checked| *checked))
                    .on_checked_change(Change::All),
            )
            .width(32),
            table::head("Name").width(Length::FillPortion(3)),
            table::head("Email").width(Length::FillPortion(5)),
            table::head("Role").width(Length::FillPortion(2)),
        ])]);
        let body = table::body(USERS.iter().enumerate().map(|(i, &(name, email, role))| {
            let selected = state.0[i];
            table::row([
                table::cell(
                    checkbox()
                        .checked(selected)
                        .on_checked_change(move |checked| Change::Row(i, checked)),
                )
                .width(32),
                table::cell(text::small(name).medium()).width(Length::FillPortion(3)),
                table::cell(email).width(Length::FillPortion(5)),
                table::cell(role).width(Length::FillPortion(2)),
            ])
            .selected(selected)
        }));
        Element::from(table([Slot::from(header), Slot::from(body)]))
    }
}
