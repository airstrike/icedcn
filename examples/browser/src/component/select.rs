// API Reference is rendered via the `select-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - select-rtl: RTL is not in scope (per CLAUDE.md).

use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedcn::Theme;
use icedcn::widget::field::Orientation::Horizontal;
use icedcn::widget::{column, field, select, switch};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Select",
    description: "Displays a list of options for the user to pick from, triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/select-demo.md",
    },
    page::Block::Md("examples/browser/snippets/select-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "align-item",
                title: "Align Item With Trigger",
                description: Some(
                    "Toggle between aligning the selected item with the trigger and placing the popup below it.",
                ),
                snippet: "examples/browser/snippets/select-align-item.md",
            },
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some(
                    "Use `select::group`, `select::label` and `select::separator` to organize items.",
                ),
                snippet: "examples/browser/snippets/select-groups.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Scrollable",
                description: Some("A select with many items that scrolls."),
                snippet: "examples/browser/snippets/select-scrollable.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/select-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use `.style(select::Invalid)` inside a field with an error."),
                snippet: "examples/browser/snippets/select-invalid.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/select-api-reference.md"),
];

/// The select page. Every select is uncontrolled.
pub struct SelectPage {
    state: page::State,
}

impl SelectPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn fruits() -> [select::Entry; 5] {
    [
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
        select::item("grapes", "Grapes"),
        select::item("pineapple", "Pineapple"),
    ]
}

fn zone(label: &str, items: &[(&str, &str)]) -> select::Entry {
    select::group(
        std::iter::once(select::label(label)).chain(
            items
                .iter()
                .map(|(value, label)| select::item(*value, *label)),
        ),
    )
}

struct Alignment;
impl<'a> Component<'a, Message, Theme> for Alignment {
    type State = Option<bool>;
    type Event = bool;
    fn update(
        &self,
        state: &mut Option<bool>,
        aligned: bool,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(aligned);
        None
    }
    fn view(&self, state: &Option<bool>) -> impl Widget<bool, Theme> + 'a {
        let aligned = state.unwrap_or(true);
        Element::from(
            column![
                field(switch().checked(aligned).on_checked_change(|value| value))
                    .orientation(Horizontal)
                    .control_last()
                    .label("Align Item")
                    .description("Toggle to align the item with the trigger."),
                select([select::group([
                    select::item("apple", "Apple"),
                    select::item("banana", "Banana"),
                    select::item("blueberry", "Blueberry"),
                    select::item("grapes", "Grapes"),
                    select::item("pineapple", "Pineapple")
                ])])
                .default_value("banana")
                .icon(icon::chevron_down)
                .position(if aligned {
                    select::Position::ItemAligned
                } else {
                    select::Position::Popper
                }),
            ]
            .width(Fill.max(320.0))
            .spacing(16),
        )
    }
}
impl page::Page for SelectPage {
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
            "demo" => select([select::group(
                std::iter::once(select::label("Fruits")).chain(fruits()),
            )])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .width(192)
            .into(),

            "groups" => select([
                select::group([
                    select::label("Fruits"),
                    select::item("apple", "Apple"),
                    select::item("banana", "Banana"),
                    select::item("blueberry", "Blueberry"),
                ]),
                select::separator(),
                select::group([
                    select::label("Vegetables"),
                    select::item("carrot", "Carrot"),
                    select::item("broccoli", "Broccoli"),
                    select::item("spinach", "Spinach"),
                ]),
            ])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .width(192)
            .into(),

            "scrollable" => select([
                zone(
                    "North America",
                    &[
                        ("est", "Eastern Standard Time"),
                        ("cst", "Central Standard Time"),
                        ("mst", "Mountain Standard Time"),
                        ("pst", "Pacific Standard Time"),
                        ("akst", "Alaska Standard Time"),
                        ("hst", "Hawaii Standard Time"),
                    ],
                ),
                zone(
                    "Europe & Africa",
                    &[
                        ("gmt", "Greenwich Mean Time"),
                        ("cet", "Central European Time"),
                        ("eet", "Eastern European Time"),
                        ("west", "Western European Summer Time"),
                        ("cat", "Central Africa Time"),
                        ("eat", "East Africa Time"),
                    ],
                ),
                zone(
                    "Asia",
                    &[
                        ("msk", "Moscow Time"),
                        ("ist", "India Standard Time"),
                        ("cst_china", "China Standard Time"),
                        ("jst", "Japan Standard Time"),
                        ("kst", "Korea Standard Time"),
                        ("ist_indonesia", "Indonesia Central Standard Time"),
                    ],
                ),
                zone(
                    "Australia & Pacific",
                    &[
                        ("awst", "Australian Western Standard Time"),
                        ("acst", "Australian Central Standard Time"),
                        ("aest", "Australian Eastern Standard Time"),
                        ("nzst", "New Zealand Standard Time"),
                        ("fjt", "Fiji Time"),
                    ],
                ),
                zone(
                    "South America",
                    &[
                        ("art", "Argentina Time"),
                        ("bot", "Bolivia Time"),
                        ("brt", "Brasilia Time"),
                        ("clt", "Chile Standard Time"),
                    ],
                ),
            ])
            .placeholder("Select a timezone")
            .icon(icon::chevron_down)
            .width(256)
            .into(),

            "disabled" => select([select::group([
                select::item("apple", "Apple"),
                select::item("banana", "Banana"),
                select::item("blueberry", "Blueberry"),
                select::item("grapes", "Grapes").disabled(true),
                select::item("pineapple", "Pineapple"),
            ])])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .disabled(true)
            .width(192)
            .into(),

            "invalid" => field(
                select([select::group([
                    select::item("apple", "Apple"),
                    select::item("banana", "Banana"),
                    select::item("blueberry", "Blueberry"),
                ])])
                .placeholder("Select a fruit")
                .icon(icon::chevron_down)
                .style(select::Invalid)
                .width(Fill),
            )
            .label("Fruit")
            .error("Please select a fruit.")
            .width(192)
            .into(),

            "align-item" => component(Alignment).boxed(),
            _ => unreachable!("unknown select preview id: {id}"),
        }
    }
}
