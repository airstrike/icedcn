//! An interactive table of keyed records with typed column definitions.
mod view;

use crate::compose::{Compose, Inherited, View};
use crate::sidebar::viewport::Width;
use iced_core::Element;
use iced_core::alignment::Horizontal;
use iced_widget::component::{Component, component};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

/// The active ordering of a sortable column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Ascending,
    Descending,
}

/// The user-editable state. Row keys and column names survive reordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value<K: Ord> {
    pub query: String,
    pub sort: Option<(&'static str, Order)>,
    pub hidden: BTreeSet<&'static str>,
    pub selected: BTreeSet<K>,
    /// Zero-based page of the filtered and sorted records.
    pub page: usize,
}

impl<K: Ord> Default for Value<K> {
    fn default() -> Self {
        Self {
            query: String::new(),
            sort: None,
            hidden: BTreeSet::new(),
            selected: BTreeSet::new(),
            page: 0,
        }
    }
}

type Compare<'a, T> = Box<dyn Fn(&T, &T) -> Ordering + 'a>;
type Filter<'a, T> = (&'a str, Box<dyn Fn(&T, &str) -> bool + 'a>);
type Actions<'a, T, M> = Rc<dyn Fn(&'a T) -> View<'a, M> + 'a>;

/// A named column with a cell renderer and optional sorting.
pub struct Column<'a, T, M> {
    label: &'static str,
    cell: Box<dyn Fn(&'a T) -> View<'a, M> + 'a>,
    compare: Option<Compare<'a, T>>,
    align: Horizontal,
}

/// Define a column. Each label must be unique within its table.
pub fn column<'a, T, M, C>(label: &'static str, cell: impl Fn(&'a T) -> C + 'a) -> Column<'a, T, M>
where
    C: Into<View<'a, M>>,
{
    Column {
        label,
        cell: Box::new(move |row| cell(row).into()),
        compare: None,
        align: Horizontal::Left,
    }
}

impl<'a, T, M> Column<'a, T, M> {
    /// Let the heading toggle ascending and descending order.
    pub fn sort_by(mut self, compare: impl Fn(&T, &T) -> Ordering + 'a) -> Self {
        self.compare = Some(Box::new(compare));
        self
    }
    /// Align this column's heading and cells.
    pub fn align_x(mut self, align: Horizontal) -> Self {
        self.align = align;
        self
    }
}

/// An uncontrolled table by default. Supply a value and handler to control it.
pub struct Table<'a, T, K: Ord, M> {
    records: &'a [T],
    key: Box<dyn Fn(&T) -> K + 'a>,
    columns: Vec<Column<'a, T, M>>,
    filter: Option<Filter<'a, T>>,
    actions: Option<Actions<'a, T, M>>,
    selectable: bool,
    page_size: usize,
    value: Option<Value<K>>,
    default: Value<K>,
    on_change: Option<Box<dyn Fn(Value<K>) -> M + 'a>>,
    inherited: Inherited,
}

/// Display records with stable, unique keys and application-defined columns.
pub fn data_table<'a, T, K: Ord, M>(
    records: &'a [T],
    key: impl Fn(&T) -> K + 'a,
    columns: impl IntoIterator<Item = Column<'a, T, M>>,
) -> Table<'a, T, K, M> {
    Table {
        records,
        key: Box::new(key),
        columns: columns.into_iter().collect(),
        filter: None,
        actions: None,
        selectable: false,
        page_size: 10,
        value: None,
        default: Value::default(),
        on_change: None,
        inherited: Inherited::default(),
    }
}

impl<'a, T, K: Ord, M> Table<'a, T, K, M> {
    /// Add a filter input. The predicate receives the original record and the trimmed query.
    pub fn filter(mut self, placeholder: &'a str, matches: impl Fn(&T, &str) -> bool + 'a) -> Self {
        self.filter = Some((placeholder, Box::new(matches)));
        self
    }
    /// Mount the supplied menu for the clicked record.
    pub fn actions(mut self, actions: impl Fn(&'a T) -> View<'a, M> + 'a) -> Self {
        self.actions = Some(Rc::new(actions));
        self
    }
    /// Show row and page selection controls.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
    /// Rows per page, at least one; defaults to ten.
    pub fn page_size(mut self, count: usize) -> Self {
        self.page_size = count.max(1);
        self
    }
    /// Drive filtering, ordering, column visibility, selection and pagination from the app.
    pub fn value(mut self, value: Value<K>) -> Self {
        self.value = Some(value);
        self
    }
    /// Seed an uncontrolled table on first mount.
    pub fn default_value(mut self, value: Value<K>) -> Self {
        self.default = value;
        self
    }
    /// Publish the complete new value for each user change.
    pub fn on_value_change(mut self, f: impl Fn(Value<K>) -> M + 'a) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
}

#[doc(hidden)]
pub struct State<K: Ord> {
    value: Option<Value<K>>,
    filtered: Vec<usize>,
    selected_count: usize,
    mobile: bool,
}
impl<K: Ord> Default for State<K> {
    fn default() -> Self {
        Self {
            value: None,
            filtered: Vec::new(),
            selected_count: 0,
            mobile: false,
        }
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub enum Event<K, M> {
    Filter(String),
    Sort(&'static str),
    Visible(&'static str, bool),
    Select(K, bool),
    PageSelection(bool),
    Page(usize),
    App(M),
    Mobile(bool),
}

impl<'a, T, K: Ord + Clone, M> Table<'a, T, K, M> {
    fn refresh(&self, value: &mut Value<K>, filtered: &mut Vec<usize>) {
        let query = value.query.trim();
        filtered.clear();
        filtered.extend((0..self.records.len()).filter(|&i| {
            query.is_empty()
                || self
                    .filter
                    .as_ref()
                    .is_none_or(|(_, accept)| accept(&self.records[i], query))
        }));
        if let Some((label, order)) = value.sort
            && let Some(compare) = self
                .columns
                .iter()
                .find(|c| c.label == label)
                .and_then(|c| c.compare.as_ref())
        {
            filtered.sort_by(|&a, &b| {
                let result = compare(&self.records[a], &self.records[b]);
                match order {
                    Order::Ascending => result,
                    Order::Descending => result.reverse(),
                }
            });
        }
        value.page = value
            .page
            .min(filtered.len().saturating_sub(1) / self.page_size);
    }
    fn selected_count(&self, value: &Value<K>, filtered: &[usize]) -> usize {
        if !self.selectable || value.selected.is_empty() {
            return 0;
        }
        filtered
            .iter()
            .filter(|&&i| value.selected.contains(&(self.key)(&self.records[i])))
            .count()
    }
    fn page<'b>(&self, value: &Value<K>, filtered: &'b [usize]) -> &'b [usize] {
        let start = value
            .page
            .saturating_mul(self.page_size)
            .min(filtered.len());
        &filtered[start..start.saturating_add(self.page_size).min(filtered.len())]
    }
}

fn set<K: Ord>(items: &mut BTreeSet<K>, key: K, enabled: bool) {
    if enabled {
        items.insert(key);
    } else {
        items.remove(&key);
    }
}

impl<'a, T: 'a, K: Ord + Clone + 'static, M: Clone + 'static> Component<'a, M, Theme>
    for Table<'a, T, K, M>
{
    type State = State<K>;
    type Event = Event<K, M>;
    fn diff(&mut self, state: &mut State<K>) {
        let value = state.value.get_or_insert_with(|| self.default.clone());
        if let Some(controlled) = &self.value {
            value.clone_from(controlled);
        }
        self.refresh(value, &mut state.filtered);
        state.selected_count = self.selected_count(value, &state.filtered);
    }
    fn update(&self, state: &mut State<K>, event: Event<K, M>, _: &Renderer) -> Option<M> {
        let reindex = matches!(event, Event::Filter(_) | Event::Sort(_));
        let recount = reindex || matches!(event, Event::Select(_, _) | Event::PageSelection(_));
        let mut value = state.value.as_ref()?.clone();
        match event {
            Event::Filter(query) => {
                value.query = query;
                value.page = 0;
            }
            Event::Sort(name) => {
                value.sort = Some((
                    name,
                    if value.sort == Some((name, Order::Ascending)) {
                        Order::Descending
                    } else {
                        Order::Ascending
                    },
                ));
                value.page = 0;
            }
            Event::Visible(name, show) => set(&mut value.hidden, name, !show),
            Event::Select(key, checked) => set(&mut value.selected, key, checked),
            Event::PageSelection(checked) => {
                for &i in self.page(&value, &state.filtered) {
                    set(&mut value.selected, (self.key)(&self.records[i]), checked);
                }
            }
            Event::Page(page) => value.page = page,
            Event::App(message) => return Some(message),
            Event::Mobile(mobile) => {
                state.mobile = mobile;
                return None;
            }
        }
        let message = self.on_change.as_ref().map(|f| f(value.clone()));
        if self.value.is_none() {
            if reindex {
                self.refresh(&mut value, &mut state.filtered);
            }
            if recount {
                state.selected_count = self.selected_count(&value, &state.filtered);
            }
            state.value = Some(value);
        }
        message
    }
    fn view(&self, state: &State<K>) -> impl Widget<Event<K, M>, Theme> + 'a {
        let mut view = self.body(
            state.value.as_ref().unwrap_or(&self.default),
            &state.filtered,
            state.selected_count,
            state.mobile,
        );
        view.inherit(&self.inherited);
        let content = view.compose();
        if self.filter.is_some() {
            Width {
                content,
                mobile: state.mobile,
                enabled: true,
                breakpoint: 768.,
                change: Event::Mobile,
            }
            .boxed()
        } else {
            content
        }
    }
}

impl<'a, T: 'a, K: Ord + Clone + 'static, M: Clone + 'static> Compose<M> for Table<'a, T, K, M> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, M, Theme, Renderer>
    where
        Self: 'b,
    {
        component(*self).boxed()
    }
}
impl<'a, T: 'a, K: Ord + Clone + 'static, M: Clone + 'static> From<Table<'a, T, K, M>>
    for View<'a, M>
{
    fn from(value: Table<'a, T, K, M>) -> Self {
        View::new(value)
    }
}
impl<'a, T: 'a, K: Ord + Clone + 'static, M: Clone + 'static> From<Table<'a, T, K, M>>
    for Element<'a, M, Theme, Renderer>
{
    fn from(value: Table<'a, T, K, M>) -> Self {
        Box::new(value).compose()
    }
}
