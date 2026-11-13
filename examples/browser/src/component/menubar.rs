use crate::{Message, icon, page};
use iced::widget::component::{Component, component};
use iced::{Element, Task, Widget};
use icedxdy::widget::{menu, menubar, row, text};
use icedxdy::{Slot, Theme};
use std::collections::BTreeSet;
pub static META: page::Meta = page::Meta {
    title: "Menubar",
    description: "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/menubar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/menubar-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "checkbox",
                title: "Checkbox",
                description: Some("Use `menubar` with checkbox items for toggleable options."),
                snippet: "examples/browser/snippets/menubar-checkbox.md",
            },
            page::Block::Example {
                id: "radio",
                title: "Radio",
                description: Some("Use radio items for single-select options."),
                snippet: "examples/browser/snippets/menubar-radio.md",
            },
            page::Block::Example {
                id: "submenu",
                title: "Submenu",
                description: Some("Use a submenu for nested menus."),
                snippet: "examples/browser/snippets/menubar-submenu.md",
            },
            page::Block::Example {
                id: "icons",
                title: "With Icons",
                description: None,
                snippet: "examples/browser/snippets/menubar-icons.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/menubar-api-reference.md"),
];
pub struct MenubarPage {
    state: page::State,
}

impl MenubarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

struct Preferences {
    checked: BTreeSet<&'static str>,
    user: &'static str,
    theme: &'static str,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            checked: BTreeSet::from(["Full URLs", "Always Show Full URLs", "Strikethrough"]),
            user: "benoit",
            theme: "system",
        }
    }
}
#[derive(Clone, Debug)]
enum Choice {
    Check(&'static str, bool),
    User(&'static str),
    Theme(&'static str),
}
#[derive(Clone, Copy)]
enum Demo {
    Demo,
    Checkbox,
    Radio,
    Submenu,
    Icons,
}
impl<'a> Component<'a, Message, Theme> for Demo {
    type State = Preferences;
    type Event = Choice;
    fn update(
        &self,
        state: &mut Preferences,
        choice: Choice,
        _: &iced::Renderer,
    ) -> Option<Message> {
        match choice {
            Choice::Check(label, true) => {
                state.checked.insert(label);
            }
            Choice::Check(label, false) => {
                state.checked.remove(label);
            }
            Choice::User(v) => state.user = v,
            Choice::Theme(v) => state.theme = v,
        }
        None
    }
    fn view(&self, state: &Preferences) -> impl Widget<Choice, Theme> + 'a {
        match self {
            Self::Demo => demo(state),
            Self::Checkbox => checkbox(state),
            Self::Radio => radio(state),
            Self::Submenu => submenu(),
            Self::Icons => icons(),
        }
    }
}
fn with_icon(
    glyph: iced::widget::Text<'static, Theme>,
    label: &'static str,
) -> Slot<'static, Choice> {
    row![glyph.size(16), text(label)]
        .spacing(6)
        .align_y(iced::Center)
        .into()
}
fn demo(state: &Preferences) -> Element<'static, Choice, Theme> {
    menubar([
        menubar::menu("File", || {
            menu::group([
                menu::group([
                    menu::item("New Tab").shortcut("⌘T").into(),
                    menu::item("New Window").shortcut("⌘N").into(),
                    menu::item("New Incognito Window").disabled(true).into(),
                ]),
                menu::separator(),
                menu::group([menu::sub(
                    "Share",
                    [menu::group([
                        menu::item("Email link").into(),
                        menu::item("Messages").into(),
                        menu::item("Notes").into(),
                    ])],
                )]),
                menu::separator(),
                menu::group([menu::item("Print...").shortcut("⌘P").into()]),
            ])
        }),
        menubar::menu("Edit", || {
            menu::group([
                menu::group([
                    menu::item("Undo").shortcut("⌘Z").into(),
                    menu::item("Redo").shortcut("⇧⌘Z").into(),
                ]),
                menu::separator(),
                menu::group([menu::sub(
                    "Find",
                    [
                        menu::group([menu::item("Search the web").into()]),
                        menu::separator(),
                        menu::group([
                            menu::item("Find...").into(),
                            menu::item("Find Next").into(),
                            menu::item("Find Previous").into(),
                        ]),
                    ],
                )]),
                menu::separator(),
                menu::group([
                    menu::item("Cut").into(),
                    menu::item("Copy").into(),
                    menu::item("Paste").into(),
                ]),
            ])
        }),
        menubar::menu("View", {
            let bookmarks_bar = state.checked.contains(&"Bookmarks Bar");
            let full_urls = state.checked.contains(&"Full URLs");
            move || {
                menu::group([
                    menu::group([
                        menu::checkbox_item("Bookmarks Bar", bookmarks_bar, |v| {
                            Choice::Check("Bookmarks Bar", v)
                        })
                        .into(),
                        menu::checkbox_item("Full URLs", full_urls, |v| {
                            Choice::Check("Full URLs", v)
                        })
                        .into(),
                    ]),
                    menu::separator(),
                    menu::group([
                        menu::item("Reload").shortcut("⌘R").inset(true).into(),
                        menu::item("Force Reload")
                            .disabled(true)
                            .shortcut("⇧⌘R")
                            .inset(true)
                            .into(),
                    ]),
                    menu::separator(),
                    menu::group([menu::item("Toggle Fullscreen").inset(true).into()]),
                    menu::separator(),
                    menu::group([menu::item("Hide Sidebar").inset(true).into()]),
                ])
            }
        })
        .width(176),
        menubar::menu("Profiles", {
            let user = state.user;
            move || {
                menu::group([
                    menu::group([
                        menu::radio_item("Andy", "andy", Some(user), Choice::User).into(),
                        menu::radio_item("Benoit", "benoit", Some(user), Choice::User).into(),
                        menu::radio_item("Luis", "Luis", Some(user), Choice::User).into(),
                    ]),
                    menu::separator(),
                    menu::group([menu::item("Edit...").inset(true).into()]),
                    menu::separator(),
                    menu::group([menu::item("Add Profile...").inset(true).into()]),
                ])
            }
        }),
    ])
    .width(288)
    .into()
}
fn checkbox(state: &Preferences) -> Element<'static, Choice, Theme> {
    menubar([
        menubar::menu("View", {
            let always_show_bookmarks_bar = state.checked.contains(&"Always Show Bookmarks Bar");
            let always_show_full_urls = state.checked.contains(&"Always Show Full URLs");
            move || {
                menu::group([
                    menu::checkbox_item(
                        "Always Show Bookmarks Bar",
                        always_show_bookmarks_bar,
                        |v| Choice::Check("Always Show Bookmarks Bar", v),
                    )
                    .into(),
                    menu::checkbox_item("Always Show Full URLs", always_show_full_urls, |v| {
                        Choice::Check("Always Show Full URLs", v)
                    })
                    .into(),
                    menu::separator(),
                    menu::item("Reload").shortcut("⌘R").inset(true).into(),
                    menu::item("Force Reload")
                        .disabled(true)
                        .shortcut("⇧⌘R")
                        .inset(true)
                        .into(),
                ])
            }
        })
        .width(256),
        menubar::menu("Format", {
            let strikethrough = state.checked.contains(&"Strikethrough");
            let code = state.checked.contains(&"Code");
            let superscript = state.checked.contains(&"Superscript");
            move || {
                menu::group([
                    menu::checkbox_item("Strikethrough", strikethrough, |v| {
                        Choice::Check("Strikethrough", v)
                    })
                    .into(),
                    menu::checkbox_item("Code", code, |v| Choice::Check("Code", v)).into(),
                    menu::checkbox_item("Superscript", superscript, |v| {
                        Choice::Check("Superscript", v)
                    })
                    .into(),
                ])
            }
        }),
    ])
    .width(288)
    .into()
}
fn radio(state: &Preferences) -> Element<'static, Choice, Theme> {
    menubar([
        menubar::menu("Profiles", {
            let user = state.user;
            move || {
                menu::group([
                    menu::group([
                        menu::radio_item("Andy", "andy", Some(user), Choice::User).into(),
                        menu::radio_item("Benoit", "benoit", Some(user), Choice::User).into(),
                        menu::radio_item("Luis", "luis", Some(user), Choice::User).into(),
                    ]),
                    menu::separator(),
                    menu::item("Edit...").inset(true).into(),
                    menu::item("Add Profile...").inset(true).into(),
                ])
            }
        }),
        menubar::menu("Theme", {
            let theme = state.theme;
            move || {
                menu::group([menu::group([
                    menu::radio_item("Light", "light", Some(theme), Choice::Theme).into(),
                    menu::radio_item("Dark", "dark", Some(theme), Choice::Theme).into(),
                    menu::radio_item("System", "system", Some(theme), Choice::Theme).into(),
                ])])
            }
        }),
    ])
    .width(288)
    .into()
}
fn submenu() -> Element<'static, Choice, Theme> {
    menubar([
        menubar::menu("File", || {
            menu::group([
                menu::sub(
                    "Share",
                    [
                        menu::item("Email link").into(),
                        menu::item("Messages").into(),
                        menu::item("Notes").into(),
                    ],
                ),
                menu::separator(),
                menu::item("Print...").shortcut("⌘P").into(),
            ])
        }),
        menubar::menu("Edit", || {
            menu::group([
                menu::item("Undo").shortcut("⌘Z").into(),
                menu::item("Redo").shortcut("⇧⌘Z").into(),
                menu::separator(),
                menu::sub(
                    "Find",
                    [
                        menu::item("Find...").into(),
                        menu::item("Find Next").into(),
                        menu::item("Find Previous").into(),
                    ],
                ),
                menu::separator(),
                menu::item("Cut").into(),
                menu::item("Copy").into(),
                menu::item("Paste").into(),
            ])
        }),
    ])
    .width(288)
    .into()
}
fn icons() -> Element<'static, Choice, Theme> {
    menubar([
        menubar::menu("File", || {
            menu::group([
                menu::item(with_icon(icon::file(), "New File"))
                    .shortcut("⌘N")
                    .into(),
                menu::item(with_icon(icon::folder(), "Open Folder")).into(),
                menu::separator(),
                menu::item(with_icon(icon::save(), "Save"))
                    .shortcut("⌘S")
                    .into(),
            ])
        }),
        menubar::menu("More", || {
            menu::group([menu::group([
                menu::item(with_icon(icon::settings(), "Settings")).into(),
                menu::item(with_icon(icon::circle_question_mark(), "Help")).into(),
                menu::separator(),
                menu::item(with_icon(icon::trash(), "Delete"))
                    .style(menu::Destructive)
                    .into(),
            ])])
        }),
    ])
    .width(288)
    .into()
}
impl page::Page for MenubarPage {
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
            "demo" => component(Demo::Demo).boxed(),
            "checkbox" => component(Demo::Checkbox).boxed(),
            "radio" => component(Demo::Radio).boxed(),
            "submenu" => component(Demo::Submenu).boxed(),
            "icons" => component(Demo::Icons).boxed(),

            _ => unreachable!("unknown menubar preview: {id}"),
        }
    }
}
