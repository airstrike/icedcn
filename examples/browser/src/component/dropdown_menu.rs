// API Reference is rendered via the `dropdown-menu-api-reference.md`
// snippet at the bottom of BLOCKS.
//
// Intentional skips:
// - dropdown-menu-rtl: RTL is not in scope (per CLAUDE.md).

use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedxdy::widget::{avatar, button, column, dropdown_menu, menu, row, text};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::component::avatar::shadcn_handle;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Dropdown Menu",
    description: "Displays a menu to the user, such as a set of actions or functions, \
                  triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/dropdown-menu-demo.md",
    },
    page::Block::Md("examples/browser/snippets/dropdown-menu-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A basic dropdown menu with labels and separators."),
                snippet: "examples/browser/snippets/dropdown-menu-basic.md",
            },
            page::Block::Example {
                id: "submenu",
                title: "Submenu",
                description: Some("Use `menu::sub(..)` to nest secondary actions."),
                snippet: "examples/browser/snippets/dropdown-menu-submenu.md",
            },
            page::Block::Example {
                id: "shortcuts",
                title: "Shortcuts",
                description: Some("Add `.shortcut(..)` to show keyboard hints."),
                snippet: "examples/browser/snippets/dropdown-menu-shortcuts.md",
            },
            page::Block::Example {
                id: "icons",
                title: "Icons",
                description: Some("Combine icons with labels for quick scanning."),
                snippet: "examples/browser/snippets/dropdown-menu-icons.md",
            },
            page::Block::Example {
                id: "checkboxes",
                title: "Checkboxes",
                description: Some("Use `menu::checkbox_item` for toggles."),
                snippet: "examples/browser/snippets/dropdown-menu-checkboxes.md",
            },
            page::Block::Example {
                id: "checkboxes-icons",
                title: "Checkboxes Icons",
                description: Some("Add icons to checkbox items."),
                snippet: "examples/browser/snippets/dropdown-menu-checkboxes-icons.md",
            },
            page::Block::Example {
                id: "radio-group",
                title: "Radio Group",
                description: Some("Use `menu::radio_item` for exclusive choices."),
                snippet: "examples/browser/snippets/dropdown-menu-radio-group.md",
            },
            page::Block::Example {
                id: "radio-icons",
                title: "Radio Icons",
                description: Some("Show radio options with icons."),
                snippet: "examples/browser/snippets/dropdown-menu-radio-icons.md",
            },
            page::Block::Example {
                id: "destructive",
                title: "Destructive",
                description: Some("Use `.style(menu::Destructive)` for irreversible actions."),
                snippet: "examples/browser/snippets/dropdown-menu-destructive.md",
            },
            page::Block::Example {
                id: "avatar",
                title: "Avatar",
                description: Some("An account switcher dropdown triggered by an avatar."),
                snippet: "examples/browser/snippets/dropdown-menu-avatar.md",
            },
            page::Block::Example {
                id: "complex",
                title: "Complex",
                description: Some("A richer example combining groups, icons, and submenus."),
                snippet: "examples/browser/snippets/dropdown-menu-complex.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/dropdown-menu-api-reference.md"),
];

pub struct DropdownMenuPage {
    state: page::State,
}

impl DropdownMenuPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn open<'a, M: Clone + 'a>() -> impl Fn(&dropdown_menu::Handle) -> Slot<'a, M> {
    |m| m.trigger(button("Open").style(button::Outline))
}

/// An item with a leading icon.
fn with_icon<'a, Message: 'a>(
    glyph: iced::widget::Text<'a, Theme>,
    label: &'a str,
) -> Slot<'a, Message> {
    row![glyph.size(16), text(label)]
        .spacing(6)
        .align_y(iced::Center)
        .into()
}

#[derive(Clone, Copy)]
struct Preferences {
    checked: [bool; 3],
    payment: &'static str,
    theme: &'static str,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            checked: [true, false, true],
            payment: "card",
            theme: "light",
        }
    }
}
#[derive(Debug, Clone)]
enum Choice {
    Checked(usize, bool),
    Payment(&'static str),
    Theme(&'static str),
}
enum PreferencesMenu {
    Notifications,
    Payment,
    Complex,
}
impl<'a> Component<'a, Message, Theme> for PreferencesMenu {
    type State = Preferences;
    type Event = Choice;
    fn update(
        &self,
        state: &mut Preferences,
        event: Choice,
        _: &iced::Renderer,
    ) -> Option<Message> {
        match event {
            Choice::Checked(i, v) => state.checked[i] = v,
            Choice::Payment(v) => state.payment = v,
            Choice::Theme(v) => state.theme = v,
        }
        None
    }
    fn view(&self, state: &Preferences) -> impl Widget<Choice, Theme> + 'a {
        match self {
            Self::Notifications => checkboxes_icons(*state),
            Self::Payment => radio_icons(*state),
            Self::Complex => complex(*state),
        }
    }
}
/// shadcn `dropdown-menu-checkboxes.tsx`: three toggles the menu shows and
/// flips, held by a user `Component` rather than app state.
struct Appearance;

impl<'a> Component<'a, Message, Theme> for Appearance {
    type State = Option<[bool; 3]>;
    type Event = (usize, bool);

    fn update(
        &self,
        state: &mut Option<[bool; 3]>,
        (index, checked): (usize, bool),
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        let mut shown = state.unwrap_or([true, false, false]);
        shown[index] = checked;
        *state = Some(shown);
        None
    }

    fn view(&self, state: &Option<[bool; 3]>) -> impl Widget<(usize, bool), Theme> + 'a {
        let [status, activity, panel] = state.unwrap_or([true, false, false]);

        Element::from(
            dropdown_menu(open(), move |_| {
                menu::group([
                    menu::label("Appearance"),
                    menu::checkbox_item("Status Bar", status, |c| (0, c)).into(),
                    menu::checkbox_item("Activity Bar", activity, |c| (1, c))
                        .disabled(true)
                        .into(),
                    menu::checkbox_item("Panel", panel, |c| (2, c)).into(),
                ])
            })
            .width(160),
        )
    }
}

/// shadcn `dropdown-menu-radio-group.tsx`: the panel position, held by a
/// user `Component`.
struct Position;

impl<'a> Component<'a, Message, Theme> for Position {
    type State = Option<&'static str>;
    type Event = &'static str;

    fn update(
        &self,
        state: &mut Option<&'static str>,
        position: &'static str,
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(position);
        None
    }

    fn view(&self, state: &Option<&'static str>) -> impl Widget<&'static str, Theme> + 'a {
        let position = state.unwrap_or("bottom");

        Element::from(
            dropdown_menu(open(), move |_| {
                menu::group([
                    menu::label("Panel Position"),
                    menu::radio_item("Top", "top", Some(position), |p| p).into(),
                    menu::radio_item("Bottom", "bottom", Some(position), |p| p).into(),
                    menu::radio_item("Right", "right", Some(position), |p| p).into(),
                ])
            })
            .width(128),
        )
    }
}

fn icons() -> Element<'static, Message, Theme> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        |_| {
            menu::group([
                menu::item(with_icon(icon::user(), "Profile")).into(),
                menu::item(with_icon(icon::credit_card(), "Billing")).into(),
                menu::item(with_icon(icon::settings(), "Settings")).into(),
                menu::separator(),
                menu::item(with_icon(icon::log_out(), "Log out"))
                    .style(menu::Destructive)
                    .into(),
            ])
        },
    )
    .into()
}

fn checkboxes_icons(state: Preferences) -> Element<'static, Choice, Theme> {
    dropdown_menu(
        |m| m.trigger(button("Notifications").style(button::Outline)),
        move |_| {
            menu::group([menu::group([
                menu::label("Notification Preferences"),
                menu::checkbox_item(
                    with_icon(icon::mail(), "Email notifications"),
                    state.checked[0],
                    |v| Choice::Checked(0, v),
                )
                .into(),
                menu::checkbox_item(
                    with_icon(icon::message_square(), "SMS notifications"),
                    state.checked[1],
                    |v| Choice::Checked(1, v),
                )
                .into(),
                menu::checkbox_item(
                    with_icon(icon::bell(), "Push notifications"),
                    state.checked[2],
                    |v| Choice::Checked(2, v),
                )
                .into(),
            ])])
        },
    )
    .width(192)
    .into()
}

fn radio_icons(state: Preferences) -> Element<'static, Choice, Theme> {
    dropdown_menu(
        |m| m.trigger(button("Payment Method").style(button::Outline)),
        move |_| {
            menu::group([menu::group([
                menu::label("Select Payment Method"),
                menu::group([
                    menu::radio_item(
                        with_icon(icon::credit_card(), "Credit Card"),
                        "card",
                        Some(state.payment),
                        Choice::Payment,
                    )
                    .into(),
                    menu::radio_item(
                        with_icon(icon::wallet(), "PayPal"),
                        "paypal",
                        Some(state.payment),
                        Choice::Payment,
                    )
                    .into(),
                    menu::radio_item(
                        with_icon(icon::building_2(), "Bank Transfer"),
                        "bank",
                        Some(state.payment),
                        Choice::Payment,
                    )
                    .into(),
                ]),
            ])])
        },
    )
    .width(224)
    .into()
}

fn avatar_menu() -> Element<'static, Message, Theme> {
    dropdown_menu(
        |m| {
            m.trigger(
                button(avatar(Some(shadcn_handle())).fallback("LR"))
                    .square()
                    .rounded()
                    .translate(false)
                    .style(button::Ghost),
            )
        },
        |_| {
            menu::group([
                menu::group([
                    menu::item(with_icon(icon::badge_check(), "Account")).into(),
                    menu::item(with_icon(icon::credit_card(), "Billing")).into(),
                    menu::item(with_icon(icon::bell(), "Notifications")).into(),
                ]),
                menu::separator(),
                menu::item(with_icon(icon::log_out(), "Sign Out")).into(),
            ])
        },
    )
    .align(dropdown_menu::Align::End)
    .into()
}

fn complex(state: Preferences) -> Element<'static, Choice, Theme> {
    dropdown_menu(
        |m| m.trigger(button("Complex Menu").style(button::Outline)),
        move |_| {
            menu::group([
                menu::group([
                    menu::label("File"),
                    menu::item(with_icon(icon::file(), "New File"))
                        .shortcut("⌘N")
                        .into(),
                    menu::item(with_icon(icon::folder(), "New Folder"))
                        .shortcut("⇧⌘N")
                        .into(),
                    menu::sub(
                        with_icon(icon::folder_open(), "Open Recent"),
                        [
                            menu::group([
                                menu::label("Recent Projects"),
                                menu::item(with_icon(icon::file_code(), "Project Alpha")).into(),
                                menu::item(with_icon(icon::file_code(), "Project Beta")).into(),
                                menu::sub(
                                    with_icon(icon::ellipsis(), "More Projects"),
                                    [
                                        menu::item(with_icon(icon::file_code(), "Project Gamma"))
                                            .into(),
                                        menu::item(with_icon(icon::file_code(), "Project Delta"))
                                            .into(),
                                    ],
                                ),
                            ]),
                            menu::separator(),
                            menu::group([menu::item(with_icon(
                                icon::folder_search(),
                                "Browse...",
                            ))
                            .into()]),
                        ],
                    ),
                    menu::separator(),
                    menu::item(with_icon(icon::save(), "Save"))
                        .shortcut("⌘S")
                        .into(),
                    menu::item(with_icon(icon::download(), "Export"))
                        .shortcut("⇧⌘E")
                        .into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::label("View"),
                    menu::checkbox_item(
                        with_icon(icon::eye(), "Show Sidebar"),
                        state.checked[0],
                        |v| Choice::Checked(0, v),
                    )
                    .into(),
                    menu::checkbox_item(
                        with_icon(icon::panels_top_left(), "Show Status Bar"),
                        state.checked[1],
                        |v| Choice::Checked(1, v),
                    )
                    .into(),
                    menu::sub(
                        with_icon(icon::palette(), "Theme"),
                        [menu::group([
                            menu::label("Appearance"),
                            menu::group([
                                menu::radio_item(
                                    with_icon(icon::sun(), "Light"),
                                    "light",
                                    Some(state.theme),
                                    Choice::Theme,
                                )
                                .into(),
                                menu::radio_item(
                                    with_icon(icon::moon(), "Dark"),
                                    "dark",
                                    Some(state.theme),
                                    Choice::Theme,
                                )
                                .into(),
                                menu::radio_item(
                                    with_icon(icon::monitor(), "System"),
                                    "system",
                                    Some(state.theme),
                                    Choice::Theme,
                                )
                                .into(),
                            ]),
                        ])],
                    ),
                ]),
                menu::separator(),
                menu::group([
                    menu::label("Account"),
                    menu::item(with_icon(icon::user(), "Profile"))
                        .shortcut("⇧⌘P")
                        .into(),
                    menu::item(with_icon(icon::credit_card(), "Billing")).into(),
                    menu::sub(
                        with_icon(icon::settings(), "Settings"),
                        [
                            menu::group([
                                menu::label("Preferences"),
                                menu::item(with_icon(icon::keyboard(), "Keyboard Shortcuts"))
                                    .into(),
                                menu::item(with_icon(icon::languages(), "Language")).into(),
                                menu::sub(
                                    with_icon(icon::bell(), "Notifications"),
                                    [menu::group([
                                        menu::label("Notification Types"),
                                        menu::checkbox_item(
                                            with_icon(icon::bell(), "Push Notifications"),
                                            state.checked[2],
                                            |v| Choice::Checked(2, v),
                                        )
                                        .into(),
                                        menu::checkbox_item(
                                            with_icon(icon::mail(), "Email Notifications"),
                                            state.checked[0],
                                            |v| Choice::Checked(0, v),
                                        )
                                        .into(),
                                    ])],
                                ),
                            ]),
                            menu::separator(),
                            menu::group([menu::item(with_icon(
                                icon::shield(),
                                "Privacy & Security",
                            ))
                            .into()]),
                        ],
                    ),
                ]),
                menu::separator(),
                menu::group([
                    menu::item(with_icon(icon::circle_question_mark(), "Help & Support")).into(),
                    menu::item(with_icon(icon::file_text(), "Documentation")).into(),
                ]),
                menu::separator(),
                menu::group([menu::item(with_icon(icon::log_out(), "Sign Out"))
                    .style(menu::Destructive)
                    .shortcut("⇧⌘Q")
                    .into()]),
            ])
        },
    )
    .width(176)
    .into()
}

impl page::Page for DropdownMenuPage {
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
            // shadcn `dropdown-menu-demo.tsx`.
            "demo" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").shortcut("⇧⌘P").into(),
                        menu::item("Billing").shortcut("⌘B").into(),
                        menu::item("Settings").shortcut("⌘S").into(),
                    ]),
                    menu::separator(),
                    menu::group([
                        menu::item("Team").into(),
                        menu::sub(
                            "Invite users",
                            [
                                menu::item("Email").into(),
                                menu::item("Message").into(),
                                menu::separator(),
                                menu::item("More...").into(),
                            ],
                        ),
                        menu::item("New Team").shortcut("⌘+T").into(),
                    ]),
                    menu::separator(),
                    menu::group([
                        menu::item("GitHub").into(),
                        menu::item("Support").into(),
                        menu::item("API").disabled(true).into(),
                    ]),
                    menu::separator(),
                    menu::item("Log out").shortcut("⇧⌘Q"),
                ]
                .width(Fill)
                .into()
            })
            .align(dropdown_menu::Align::Start)
            .width(160)
            .into(),

            // shadcn `dropdown-menu-basic.tsx`.
            "basic" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").into(),
                        menu::item("Billing").into(),
                        menu::item("Settings").into(),
                    ]),
                    menu::separator(),
                    menu::item("GitHub"),
                    menu::item("Support"),
                    menu::item("API").disabled(true),
                ]
                .width(Fill)
                .into()
            })
            .into(),

            // shadcn `dropdown-menu-submenu.tsx`.
            "submenu" => dropdown_menu(open(), |_| {
                menu::group([
                    menu::item("Team").into(),
                    menu::sub(
                        "Invite users",
                        [
                            menu::item("Email").into(),
                            menu::item("Message").into(),
                            menu::sub(
                                "More options",
                                [
                                    menu::item("Calendly").into(),
                                    menu::item("Slack").into(),
                                    menu::separator(),
                                    menu::item("Webhook").into(),
                                ],
                            ),
                            menu::separator(),
                            menu::item("Advanced...").into(),
                        ],
                    ),
                    menu::item("New Team").shortcut("⌘+T").into(),
                ])
            })
            .into(),

            // shadcn `dropdown-menu-shortcuts.tsx`.
            "shortcuts" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").shortcut("⇧⌘P").into(),
                        menu::item("Billing").shortcut("⌘B").into(),
                        menu::item("Settings").shortcut("⌘S").into(),
                    ]),
                    menu::separator(),
                    menu::item("Log out").shortcut("⇧⌘Q"),
                ]
                .width(Fill)
                .into()
            })
            .into(),

            "checkboxes" => component(Appearance).boxed(),

            "radio-group" => component(Position).boxed(),

            // shadcn `dropdown-menu-destructive.tsx`.
            "destructive" => dropdown_menu(
                |m| m.trigger(button("Actions").style(button::Outline)),
                |_| {
                    column![
                        menu::group([
                            menu::item(with_icon(icon::pencil(), "Edit")).into(),
                            menu::item(with_icon(icon::share(), "Share")).into(),
                        ]),
                        menu::separator(),
                        menu::item(with_icon(icon::trash_2(), "Delete")).style(menu::Destructive),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .into(),

            "icons" => icons(),
            "checkboxes-icons" => component(PreferencesMenu::Notifications).boxed(),
            "radio-icons" => component(PreferencesMenu::Payment).boxed(),
            "avatar" => avatar_menu(),
            "complex" => component(PreferencesMenu::Complex).boxed(),
            _ => unreachable!("unknown dropdown menu preview id: {id}"),
        }
    }
}
