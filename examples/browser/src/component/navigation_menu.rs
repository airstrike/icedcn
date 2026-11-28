use crate::{Message, page};
use iced::widget::responsive;
use iced::{Fill, Task, Widget};
use icedcn::widget::{column, container, menu, navigation_menu, row, text};
use icedcn::{Theme, View};
pub static META: page::Meta = page::Meta {
    title: "Navigation Menu",
    description: "A collection of links for navigating an application.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/navigation-menu-demo.md",
    },
    page::Block::Md("examples/browser/snippets/navigation-menu-composition.md"),
    page::Block::Md("examples/browser/snippets/navigation-menu-link-component.md"),
    page::Block::Md("examples/browser/snippets/navigation-menu-api-reference.md"),
];
pub struct NavigationMenuPage {
    state: page::State,
}

impl NavigationMenuPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn doc_link(
    label: &'static str,
    description: &'static str,
    url: &'static str,
) -> View<'static, Message> {
    menu::item(
        column![
            text(label).medium().size(14),
            text::muted(description).size(14)
        ]
        .spacing(4),
    )
    .on_select(Message::OpenUrl(url))
    .into()
}
const COMPONENTS: &[(&str, &str, &str)] = &[
    (
        "Alert Dialog",
        "A modal dialog that interrupts the user with important content and expects a response.",
        "icedcn://component/alert-dialog",
    ),
    (
        "Hover Card",
        "For sighted users to preview content available behind a link.",
        "icedcn://component/hover-card",
    ),
    (
        "Progress",
        "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
        "icedcn://component/progress",
    ),
    (
        "Scroll Area",
        "Scroll through content inside a fixed viewport.",
        "icedcn://component/scroll-area",
    ),
    (
        "Tabs",
        "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
        "icedcn://component/tabs",
    ),
    (
        "Tooltip",
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
        "icedcn://component/tooltip",
    ),
];

fn navigation(wide: bool) -> View<'static, Message> {
    let mut entries = vec![
        navigation_menu::menu("Getting started", || {
            column![
                doc_link(
                    "Introduction",
                    "Reusable widgets for iced applications.",
                    "https://github.com/airstrike/icedcn"
                ),
                doc_link(
                    "Installation",
                    "How to install dependencies and structure your app.",
                    "https://github.com/airstrike/icedcn"
                ),
                doc_link(
                    "Typography",
                    "Styles for headings, paragraphs, lists...etc",
                    "icedcn://component/typography"
                )
            ]
            .into()
        })
        .width(384),
    ];
    if wide {
        entries.push(
            navigation_menu::menu("Components", || {
                let pairs = COMPONENTS.chunks(2).map(|pair| {
                    row::Row::with_children(pair.iter().map(|&(title, description, url)| {
                        container(doc_link(title, description, url))
                            .width(Fill)
                            .into()
                    }))
                    .spacing(8)
                    .into()
                });
                column::Column::with_children(pairs).spacing(8).into()
            })
            .width(500),
        );
    }
    entries.push(navigation_menu::link(
        "Docs",
        Message::OpenUrl("https://github.com/airstrike/icedcn"),
    ));
    navigation_menu(entries).into()
}
impl page::Page for NavigationMenuPage {
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
            "demo" => responsive(|size| navigation(size.width >= 400.).compose())
                .width(Fill)
                .boxed(),
            _ => unreachable!("unknown navigation-menu preview: {id}"),
        }
    }
}
