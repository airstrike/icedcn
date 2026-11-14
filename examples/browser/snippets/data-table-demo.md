```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedcn::Slot;
use icedcn::widget::{column, data_table, menu, text};

pub struct Payment<'a> {
    pub id: &'a str,
    pub status: &'a str,
    pub email: &'a str,
    pub amount: f64,
}

pub fn payments<'a, M: Clone + 'static>(
    records: &'a [Payment<'a>],
    copy_id: impl Fn(String) -> M + 'a,
) -> Slot<'a, M> {
    data_table(
        records,
        |p| p.id.to_owned(),
        [
            data_table::column("Status", |p: &Payment| text::small(p.status)),
            data_table::column("Email", |p: &Payment| text::small(p.email.to_lowercase())).sort_by(
                |a, b| {
                    a.email
                        .to_ascii_lowercase()
                        .cmp(&b.email.to_ascii_lowercase())
                },
            ),
            data_table::column("Amount", |p: &Payment| {
                text::small(format!("${:.2}", p.amount)).medium()
            })
            .align_x(Right),
        ],
    )
    .filter("Filter emails...", |p, query| {
        p.email.to_lowercase().contains(&query.to_lowercase())
    })
    .selectable(true)
    .actions(move |payment| {
        column![
            menu::label("Actions"),
            menu::item("Copy payment ID").on_select(copy_id(payment.id.to_owned())),
            menu::separator(),
            menu::item("View customer"),
            menu::item("View payment details"),
        ]
        .width(Fill)
        .into()
    })
    .into()
}
```
