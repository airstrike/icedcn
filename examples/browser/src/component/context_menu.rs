// API Reference is rendered via the `context-menu-api-reference.md` snippet
// at the bottom of BLOCKS.
//
// The trigger area's `border-dashed` draws solid: iced has no dashed
// borders.
//
// Intentional skips:
// - context-menu-rtl: RTL is not in scope (per CLAUDE.md).

use iced::widget::Text;
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{column, container, context_menu, menu, row, text};
use icedxdy::{Slot, Theme, radius};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Context Menu",
    description: "Displays a menu to the user, such as a set of actions or functions, \
                  triggered by a right click.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/context-menu-demo.md",
    },
    page::Block::Md("examples/browser/snippets/context-menu-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple context menu with a few actions."),
                snippet: "examples/browser/snippets/context-menu-basic.md",
            },
            page::Block::Example {
                id: "submenu",
                title: "Submenu",
                description: Some("Use `ContextMenuSub` to nest secondary actions."),
                snippet: "examples/browser/snippets/context-menu-submenu.md",
            },
            page::Block::Example {
                id: "shortcuts",
                title: "Shortcuts",
                description: Some("Add `ContextMenuShortcut` to show keyboard hints."),
                snippet: "examples/browser/snippets/context-menu-shortcuts.md",
            },
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some("Group related actions and separate them with dividers."),
                snippet: "examples/browser/snippets/context-menu-groups.md",
            },
            page::Block::Example {
                id: "icons",
                title: "Icons",
                description: Some("Combine icons with labels for quick scanning."),
                snippet: "examples/browser/snippets/context-menu-icons.md",
            },
            page::Block::Example {
                id: "checkboxes",
                title: "Checkboxes",
                description: Some("Use `ContextMenuCheckboxItem` for toggles."),
                snippet: "examples/browser/snippets/context-menu-checkboxes.md",
            },
            page::Block::Example {
                id: "radio",
                title: "Radio",
                description: Some("Use `ContextMenuRadioItem` for exclusive choices."),
                snippet: "examples/browser/snippets/context-menu-radio.md",
            },
            page::Block::Example {
                id: "destructive",
                title: "Destructive",
                description: Some(
                    "Use `variant=\"destructive\"` to style the menu item as destructive.",
                ),
                snippet: "examples/browser/snippets/context-menu-destructive.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/context-menu-api-reference.md"),
];

pub struct ContextMenuPage {
    state: page::State,
}

impl ContextMenuPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The `aspect-video max-w-xs rounded-xl border border-dashed` area.
fn area<'a, M: 'a>() -> Slot<'a, M> {
    container(text("Right click here"))
        .center(Fill)
        .width(320)
        .height(180)
        .style(|theme: &Theme| Style {
            border: Border {
                width: 1.0,
                color: theme.neutral.border,
                radius: radius::XL.into(),
            },
            ..Style::default()
        })
        .into()
}

fn with_icon<'a, M: 'a>(glyph: Text<'a, Theme>, label: &'a str) -> Slot<'a, M> {
    row![glyph.size(16), text(label)]
        .spacing(6)
        .align_y(iced::Center)
        .into()
}

/// The demo's toggles and person, held by a user `Component`.
#[derive(Debug, Clone, Copy)]
enum Choice {
    Checked(usize, bool),
    Theme(&'static str),
    Bookmarks(bool),
    FullUrls(bool),
    Person(&'static str),
}

#[derive(Clone, Copy)]
struct Preferences {
    checked: [bool; 3],
    person: &'static str,
    theme: &'static str,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            checked: [true, false, true],
            person: "pedro",
            theme: "light",
        }
    }
}
enum Demo {
    Browser,
    Checkboxes,
    Radio,
}

impl<'a> Component<'a, Message, Theme> for Demo {
    type State = Preferences;
    type Event = Choice;

    fn update(
        &self,
        state: &mut Preferences,
        choice: Choice,
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        match choice {
            Choice::Bookmarks(on) => state.checked[0] = on,
            Choice::FullUrls(on) => state.checked[1] = on,
            Choice::Person(who) => state.person = who,
            Choice::Checked(i, on) => state.checked[i] = on,
            Choice::Theme(choice) => state.theme = choice,
        }

        None
    }

    fn view(&self, state: &Preferences) -> impl Widget<Choice, Theme> + 'a {
        let state = *state;
        let [bookmarks, full_urls, _] = state.checked;
        let person = state.person;
        match self {
            Self::Checkboxes => return Element::from(checkboxes(state)),
            Self::Radio => return Element::from(radio(state)),
            Self::Browser => {}
        }

        Element::from(
            context_menu(
                |_| area(),
                move |_| {
                    column![
                        menu::group([
                            menu::item("Back").shortcut("⌘[").into(),
                            menu::item("Forward").shortcut("⌘]").disabled(true).into(),
                            menu::item("Reload").shortcut("⌘R").into(),
                            menu::sub(
                                "More Tools",
                                [
                                    menu::item("Save Page...").into(),
                                    menu::item("Create Shortcut...").into(),
                                    menu::item("Name Window...").into(),
                                    menu::separator(),
                                    menu::item("Developer Tools").into(),
                                    menu::separator(),
                                    menu::item("Delete").style(menu::Destructive).into(),
                                ],
                            ),
                        ]),
                        menu::separator(),
                        menu::group([
                            menu::checkbox_item("Show Bookmarks", bookmarks, Choice::Bookmarks)
                                .into(),
                            menu::checkbox_item("Show Full URLs", full_urls, Choice::FullUrls)
                                .into(),
                        ]),
                        menu::separator(),
                        menu::group([
                            menu::label("People"),
                            menu::radio_item("Pedro Duarte", "pedro", Some(person), Choice::Person)
                                .into(),
                            menu::radio_item("Colm Tuite", "colm", Some(person), Choice::Person)
                                .into(),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .width(192),
        )
    }
}

fn submenu() -> Element<'static, Message, Theme> {
    context_menu(
        |_| area(),
        |_| {
            menu::group([
                menu::group([
                    menu::item("Copy").shortcut("⌘C").into(),
                    menu::item("Cut").shortcut("⌘X").into(),
                ]),
                menu::sub(
                    "More Tools",
                    [
                        menu::group([
                            menu::item("Save Page...").into(),
                            menu::item("Create Shortcut...").into(),
                            menu::item("Name Window...").into(),
                        ]),
                        menu::separator(),
                        menu::group([menu::item("Developer Tools").into()]),
                        menu::separator(),
                        menu::group([menu::item("Delete").style(menu::Destructive).into()]),
                    ],
                ),
            ])
        },
    )
    .into()
}

fn shortcuts() -> Element<'static, Message, Theme> {
    context_menu(
        |_| area(),
        |_| {
            menu::group([
                menu::group([
                    menu::item("Back").shortcut("⌘[").into(),
                    menu::item("Forward").disabled(true).shortcut("⌘]").into(),
                    menu::item("Reload").shortcut("⌘R").into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::item("Save").shortcut("⌘S").into(),
                    menu::item("Save As...").shortcut("⇧⌘S").into(),
                ]),
            ])
        },
    )
    .into()
}

fn groups() -> Element<'static, Message, Theme> {
    context_menu(
        |_| area(),
        |_| {
            menu::group([
                menu::group([
                    menu::label("File"),
                    menu::item("New File").shortcut("⌘N").into(),
                    menu::item("Open File").shortcut("⌘O").into(),
                    menu::item("Save").shortcut("⌘S").into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::label("Edit"),
                    menu::item("Undo").shortcut("⌘Z").into(),
                    menu::item("Redo").shortcut("⇧⌘Z").into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::item("Cut").shortcut("⌘X").into(),
                    menu::item("Copy").shortcut("⌘C").into(),
                    menu::item("Paste").shortcut("⌘V").into(),
                ]),
                menu::separator(),
                menu::group([menu::item("Delete")
                    .style(menu::Destructive)
                    .shortcut("⌫")
                    .into()]),
            ])
        },
    )
    .into()
}

fn icons() -> Element<'static, Message, Theme> {
    context_menu(
        |_| area(),
        |_| {
            menu::group([
                menu::group([
                    menu::item(with_icon(icon::copy(), "Copy")).into(),
                    menu::item(with_icon(icon::scissors(), "Cut")).into(),
                    menu::item(with_icon(icon::clipboard_paste(), "Paste")).into(),
                ]),
                menu::separator(),
                menu::group([menu::item(with_icon(icon::trash(), "Delete"))
                    .style(menu::Destructive)
                    .into()]),
            ])
        },
    )
    .into()
}

fn checkboxes(state: Preferences) -> Element<'static, Choice, Theme> {
    context_menu(
        |_| area(),
        move |_| {
            menu::group([menu::group([
                menu::checkbox_item("Show Bookmarks Bar", state.checked[0], |v| {
                    Choice::Checked(0, v)
                })
                .into(),
                menu::checkbox_item("Show Full URLs", state.checked[1], |v| {
                    Choice::Checked(1, v)
                })
                .into(),
                menu::checkbox_item("Show Developer Tools", state.checked[2], |v| {
                    Choice::Checked(2, v)
                })
                .into(),
            ])])
        },
    )
    .into()
}

fn radio(state: Preferences) -> Element<'static, Choice, Theme> {
    context_menu(
        |_| area(),
        move |_| {
            menu::group([
                menu::group([
                    menu::label("People"),
                    menu::group([
                        menu::radio_item(
                            "Pedro Duarte",
                            "pedro",
                            Some(state.person),
                            Choice::Person,
                        )
                        .into(),
                        menu::radio_item("Colm Tuite", "colm", Some(state.person), Choice::Person)
                            .into(),
                    ]),
                ]),
                menu::separator(),
                menu::group([
                    menu::label("Theme"),
                    menu::group([
                        menu::radio_item("Light", "light", Some(state.theme), Choice::Theme).into(),
                        menu::radio_item("Dark", "dark", Some(state.theme), Choice::Theme).into(),
                        menu::radio_item("System", "system", Some(state.theme), Choice::Theme)
                            .into(),
                    ]),
                ]),
            ])
        },
    )
    .into()
}

fn destructive() -> Element<'static, Message, Theme> {
    context_menu(
        |_| area(),
        |_| {
            menu::group([
                menu::group([
                    menu::item(with_icon(icon::pencil(), "Edit")).into(),
                    menu::item(with_icon(icon::share(), "Share")).into(),
                ]),
                menu::separator(),
                menu::group([menu::item(with_icon(icon::trash(), "Delete"))
                    .style(menu::Destructive)
                    .into()]),
            ])
        },
    )
    .into()
}

impl page::Page for ContextMenuPage {
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
            // shadcn `context-menu-demo.tsx`.
            "demo" => component(Demo::Browser).boxed(),

            // shadcn `context-menu-basic.tsx`.
            "basic" => context_menu(
                |_| area(),
                |_| {
                    menu::group([
                        menu::item("Back").into(),
                        menu::item("Forward").disabled(true).into(),
                        menu::item("Reload").into(),
                    ])
                },
            )
            .into(),

            "submenu" => submenu(),
            "shortcuts" => shortcuts(),
            "groups" => groups(),
            "icons" => icons(),
            "checkboxes" => component(Demo::Checkboxes).boxed(),
            "radio" => component(Demo::Radio).boxed(),
            "destructive" => destructive(),
            _ => unreachable!("unknown context menu preview id: {id}"),
        }
    }
}
