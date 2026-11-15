use iced::widget::component::{Component, component};
use iced::{Element, Widget};
use icedcn::Slot;
use icedcn::Theme;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{checkbox, table, text};

use crate::Message;

pub(super) fn preview<'a>() -> Element<'a, Message, Theme> {
    component(Selection).boxed()
}

struct Selection;
struct Rows([User<'static>; 4]);

impl Default for Rows {
    fn default() -> Self {
        Self(
            [
                ("Sarah Chen", "sarah.chen@example.com", "Admin", true),
                (
                    "Marcus Rodriguez",
                    "marcus.rodriguez@example.com",
                    "User",
                    false,
                ),
                ("Priya Patel", "priya.patel@example.com", "User", false),
                ("David Kim", "david.kim@example.com", "Editor", false),
            ]
            .map(|(name, email, role, selected)| User {
                name,
                email,
                role,
                selected,
            }),
        )
    }
}

impl<'a> Component<'a, Message, Theme> for Selection {
    type State = Rows;
    type Event = Change;

    fn update(&self, state: &mut Rows, change: Change, _: &iced::Renderer) -> Option<Message> {
        change.apply(&mut state.0);
        None
    }

    fn view(&self, state: &Rows) -> impl Widget<Change, Theme> + 'a {
        users(&state.0).compose()
    }
}

pub struct User<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub role: &'a str,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub enum Change {
    All(bool),
    Row(usize, bool),
}

impl Change {
    pub fn apply(self, users: &mut [User<'_>]) {
        match self {
            Self::All(value) => users.iter_mut().for_each(|user| user.selected = value),
            Self::Row(index, value) => {
                if let Some(user) = users.get_mut(index) {
                    user.selected = value;
                }
            }
        }
    }
}

pub fn users<'a>(rows: &[User<'a>]) -> Slot<'a, Change> {
    let all = !rows.is_empty() && rows.iter().all(|user| user.selected);
    let select_all = checkbox().checked(all).on_checked_change(Change::All);
    let header = table::header([table::row([
        table::head(select_all).padding([0, 0, 0, 8]),
        table::head("Name"),
        table::head("Email"),
        table::head("Role"),
    ])]);
    let body = table::body(rows.iter().enumerate().map(|(i, user)| {
        let check = checkbox()
            .checked(user.selected)
            .on_checked_change(move |value| Change::Row(i, value));
        table::row([
            table::cell(check).padding([8, 0, 8, 8]),
            table::cell(text(user.name).medium()),
            table::cell(user.email),
            table::cell(user.role),
        ])
        .selected(user.selected)
    }));
    table([table::grid([Auto; 4]).header(header).body(body)]).into()
}
