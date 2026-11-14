use iced::widget::component::{Component, component};
use iced::widget::scrollable;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::{Element, Fill, Pixels, Widget};
use icedxdy::Theme;
use icedxdy::widget::table::Track;
use icedxdy::widget::{checkbox, table, text};

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
            .padding([0, 0, 0, 8]),
            table::head(text::small("Name").medium().line_height(Pixels(20.0))),
            table::head(text::small("Email").medium().line_height(Pixels(20.0))),
            table::head(text::small("Role").medium().line_height(Pixels(20.0))),
        ])]);
        let body = table::body(USERS.iter().enumerate().map(|(i, &(name, email, role))| {
            let selected = state.0[i];
            table::row([
                table::cell(
                    checkbox()
                        .checked(selected)
                        .on_checked_change(move |checked| Change::Row(i, checked)),
                )
                .padding([8, 0, 8, 8]),
                table::cell(text::small(name).medium().line_height(Pixels(20.0))),
                table::cell(text::small(email).line_height(Pixels(20.0))),
                table::cell(text::small(role).line_height(Pixels(20.0))),
            ])
            .selected(selected)
        }));
        scrollable(Element::from(
            table::grid([Track::Auto; 4]).header(header).body(body),
        ))
        .width(Fill)
        .direction(Direction::Horizontal(Scrollbar::default()))
    }
}
