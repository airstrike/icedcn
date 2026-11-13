use crate::icon;
use crate::{Message, page};
use iced::Task;
use iced::alignment::Horizontal::Right;
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedxdy::Theme;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::table::Track;
use icedxdy::widget::{
    button, checkbox, column, container, dropdown_menu, input, menu, row, space, table, text,
};
use icedxdy::{Size, Slot};
use std::collections::BTreeSet;

pub static META: page::Meta = page::Meta {
    title: "Data Table",
    description: "An interactive data table with sorting, filtering, column visibility and row selection.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/data-table-demo.md",
    },
    page::Block::Md("examples/browser/snippets/data-table-composition.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-basic-table.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-cell-formatting.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-row-actions.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-pagination.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-sorting.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-filtering.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-visibility.md"),
    page::Block::Md("examples/browser/snippets/data-table-guide-row-selection.md"),
];
const PAYMENTS: [Payment; 5] = [
    Payment {
        id: "m5gr84i9",
        amount: 316,
        status: "Success",
        email: "ken99@example.com",
    },
    Payment {
        id: "3u1reuv4",
        amount: 242,
        status: "Success",
        email: "abe45@example.com",
    },
    Payment {
        id: "derv1ws0",
        amount: 837,
        status: "Processing",
        email: "monserrat44@example.com",
    },
    Payment {
        id: "5kma53ae",
        amount: 874,
        status: "Success",
        email: "silas22@example.com",
    },
    Payment {
        id: "bhqecj4p",
        amount: 721,
        status: "Failed",
        email: "carmella@example.com",
    },
];
#[derive(Clone, Copy)]
struct Payment {
    id: &'static str,
    amount: u32,
    status: &'static str,
    email: &'static str,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Column {
    Status,
    Email,
    Amount,
}
impl Column {
    const ALL: [Self; 3] = [Self::Status, Self::Email, Self::Amount];
    fn label(self) -> &'static str {
        match self {
            Self::Status => "Status",
            Self::Email => "Email",
            Self::Amount => "Amount",
        }
    }
    fn track(self) -> Track {
        match self {
            Self::Email => Track::Flex(1),
            _ => Track::Auto,
        }
    }
}
#[derive(Clone, Debug)]
enum Event {
    Filter(String),
    Sort,
    Visible(Column, bool),
    Select(usize, bool),
    SelectPage(bool),
    Previous,
    Next,
    Copy(&'static str),
}
struct Model {
    query: String,
    ascending: Option<bool>,
    selected: BTreeSet<usize>,
    hidden: BTreeSet<Column>,
    filtered: Vec<usize>,
    page: usize,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            query: String::new(),
            ascending: None,
            selected: BTreeSet::new(),
            hidden: BTreeSet::new(),
            filtered: (0..PAYMENTS.len()).collect(),
            page: 0,
        }
    }
}
impl Model {
    const PAGE_SIZE: usize = 10;
    fn refresh(&mut self) {
        let query = self.query.to_lowercase();
        self.filtered = (0..PAYMENTS.len())
            .filter(|&i| PAYMENTS[i].email.contains(&query))
            .collect();
        if let Some(ascending) = self.ascending {
            self.filtered.sort_by_key(|&i| PAYMENTS[i].email);
            if !ascending {
                self.filtered.reverse();
            }
        }
        self.page = 0;
    }
    fn current(&self) -> &[usize] {
        let start = (self.page * Self::PAGE_SIZE).min(self.filtered.len());
        &self.filtered[start..(start + Self::PAGE_SIZE).min(self.filtered.len())]
    }
    fn can_advance(&self) -> bool {
        (self.page + 1) * Self::PAGE_SIZE < self.filtered.len()
    }
    fn page_selection(&self) -> checkbox::State {
        let ids = self.current();
        let count = ids.iter().filter(|i| self.selected.contains(i)).count();
        if count == 0 {
            checkbox::State::Unchecked
        } else if count == ids.len() {
            checkbox::State::Checked
        } else {
            checkbox::State::Indeterminate
        }
    }
    fn apply(&mut self, event: Event) -> Option<Message> {
        match event {
            Event::Filter(query) => {
                self.query = query;
                self.refresh();
            }
            Event::Sort => {
                self.ascending = Some(!self.ascending.unwrap_or(false));
                self.refresh();
            }
            Event::Select(i, select) => {
                if select {
                    self.selected.insert(i);
                } else {
                    self.selected.remove(&i);
                }
            }
            Event::SelectPage(select) => {
                let ids = self.current().to_vec();
                for i in ids {
                    self.apply(Event::Select(i, select));
                }
            }
            Event::Visible(col, visible) => {
                if visible {
                    self.hidden.remove(&col);
                } else {
                    self.hidden.insert(col);
                }
            }
            Event::Previous => self.page = self.page.saturating_sub(1),
            Event::Next if self.can_advance() => self.page += 1,
            Event::Next => {}
            Event::Copy(id) => return Some(Message::CopyText(id.to_owned())),
        }
        None
    }
}
struct Payments;
impl<'a> Component<'a, Message, Theme> for Payments {
    type State = Model;
    type Event = Event;
    fn update(&self, state: &mut Model, event: Event, _: &iced::Renderer) -> Option<Message> {
        state.apply(event)
    }
    fn view(&self, state: &Model) -> impl Widget<Event, Theme> + 'a {
        payments(state)
    }
}
fn actions(id: &'static str) -> Slot<'static, Event> {
    dropdown_menu(
        |m| {
            m.trigger(
                button(icon::ellipsis())
                    .style(button::Ghost)
                    .size(Size::Xs)
                    .square(),
            )
        },
        move |_| {
            column![
                menu::label("Actions"),
                menu::item("Copy payment ID").on_select(Event::Copy(id)),
                menu::separator(),
                menu::item("View customer"),
                menu::item("View payment details")
            ]
            .width(Fill)
            .into()
        },
    )
    .align(dropdown_menu::Align::End)
    .width(160)
    .into()
}
fn payments(state: &Model) -> Element<'static, Event, Theme> {
    let hidden = state.hidden.clone();
    let columns = dropdown_menu(
        |m| {
            m.trigger(
                button(row![text::small("Columns"), icon::chevron_down()].spacing(6))
                    .style(button::Outline),
            )
        },
        move |_| {
            column(Column::ALL.into_iter().map(|col| {
                menu::checkbox_item(col.label(), !hidden.contains(&col), move |visible| {
                    Event::Visible(col, visible)
                })
                .into()
            }))
            .width(Fill)
            .into()
        },
    )
    .align(dropdown_menu::Align::End);
    let visible: Vec<_> = Column::ALL
        .into_iter()
        .filter(|c| !state.hidden.contains(c))
        .collect();
    let mut tracks = vec![Track::Fixed(40.0)];
    tracks.extend(visible.iter().map(|c| c.track()));
    tracks.push(Track::Auto);
    let mut headings = vec![table::head(
        checkbox()
            .checked(state.page_selection())
            .on_checked_change(Event::SelectPage),
    )];
    for col in &visible {
        headings.push(match col {
            Column::Email => table::head(
                button(row![text::small("Email"), icon::arrow_up_down()].spacing(6))
                    .style(button::Ghost)
                    .on_press(Event::Sort),
            )
            .padding(0),
            Column::Amount => table::head("Amount").align_x(Right),
            Column::Status => table::head("Status"),
        });
    }
    headings.push(table::head(""));
    let mut rows = Vec::new();
    for &i in state.current() {
        let p = PAYMENTS[i];
        let selected = state.selected.contains(&i);
        let mut cells = vec![table::cell(
            checkbox()
                .checked(selected)
                .on_checked_change(move |s| Event::Select(i, s)),
        )];
        for col in &visible {
            cells.push(match col {
                Column::Status => table::cell(text::small(p.status)),
                Column::Email => table::cell(text::small(p.email)),
                Column::Amount => {
                    table::cell(text::small(format!("${}.00", p.amount)).medium()).align_x(Right)
                }
            });
        }
        cells.push(table::cell(actions(p.id)).align_x(Right));
        rows.push(table::row(cells).selected(selected));
    }
    if rows.is_empty() {
        rows.push(table::row([table::cell("No results.")
            .span(tracks.len())
            .height(96)
            .align_x(iced::Center)]));
    }
    let grid = table::grid(tracks)
        .header(table::header([table::row(headings)]))
        .body(table::body(rows));
    let count = state
        .filtered
        .iter()
        .filter(|i| state.selected.contains(i))
        .count();
    let summary = format!("{count} of {} row(s) selected.", state.filtered.len());
    column![
        row![
            container(
                input("Filter emails...")
                    .value(state.query.clone())
                    .on_input(Event::Filter)
                    .width(Fill)
            )
            .width(Fill)
            .max_width(384),
            space::horizontal(),
            columns
        ]
        .width(Fill)
        .spacing(8),
        container(grid)
            .padding(1)
            .clip(true)
            .width(Fill)
            .style(|t| Style {
                border: Border {
                    color: t.neutral.border,
                    width: 1.0,
                    radius: 8.into()
                },
                ..Style::default()
            }),
        row![
            text::muted(summary).size(14.0).width(Fill),
            button("Previous")
                .size(Size::Sm)
                .style(button::Outline)
                .disabled(state.page == 0)
                .on_press(Event::Previous),
            button("Next")
                .size(Size::Sm)
                .style(button::Outline)
                .disabled(!state.can_advance())
                .on_press(Event::Next)
        ]
        .width(Fill)
        .spacing(8)
    ]
    .spacing(16)
    .width(Fill)
    .into()
}

pub struct DataTablePage {
    state: page::State,
}

impl DataTablePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for DataTablePage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => component(Payments).boxed().into(),
            _ => unreachable!("unknown data-table preview id: {id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filtering_and_sorting_preserve_selection_by_record() {
        let mut model = Model::default();
        model.apply(Event::Select(0, true));
        model.apply(Event::Sort);
        assert_eq!(model.current()[0], 1);
        model.apply(Event::Filter("KEN99".into()));
        assert_eq!(model.current(), &[0]);
        assert_eq!(model.page_selection(), checkbox::State::Checked);
        model.apply(Event::Filter(String::new()));
        assert_eq!(model.page_selection(), checkbox::State::Indeterminate);
        model.apply(Event::SelectPage(true));
        assert_eq!(model.selected.len(), PAYMENTS.len());
        model.apply(Event::Filter("nothing".into()));
        assert!(model.current().is_empty());
        assert!(!model.can_advance());
    }
}
