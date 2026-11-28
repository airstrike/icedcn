```rust
use icedcn::View;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{checkbox, table, text};

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

pub fn users<'a>(rows: &[User<'a>]) -> View<'a, Change> {
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
```
