use iced::widget::image;
// RTL is outside the supported examples.
use iced::widget::Text;
use iced::widget::component::{Component, component};
use iced::{Element, Task, Widget};

use icedcn::widget::{avatar, button, dropdown_menu, menu, menu_group, menu_sub};
use icedcn::{Slot, Theme};

use crate::Message;
use crate::component::avatar::alex_handle;
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
    page::Block::Md("examples/browser/snippets/dropdown-menu-installation.md"),
    page::Block::Md("examples/browser/snippets/dropdown-menu-usage.md"),
    page::Block::Md("examples/browser/snippets/dropdown-menu-composition.md"),
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

/// The preference values survive the popup subtree being unmounted on selection.
struct Local<T> {
    initial: T,
    view: fn(T) -> Slot<'static, T>,
}

fn local<T: Copy + 'static>(
    initial: T,
    view: fn(T) -> Slot<'static, T>,
) -> Element<'static, Message, Theme> {
    component(Local { initial, view }).boxed()
}
impl<T: Copy + 'static> Component<'static, Message, Theme> for Local<T> {
    type State = Option<T>;
    type Event = T;
    fn update(&self, state: &mut Option<T>, value: T, _: &iced::Renderer) -> Option<Message> {
        *state = Some(value);
        None
    }
    fn view(&self, state: &Option<T>) -> impl Widget<T, Theme> + 'static {
        (self.view)(state.unwrap_or(self.initial)).compose()
    }
}

#[derive(Debug, Clone, Copy)]
struct Preferences {
    email: bool,
    sms: bool,
    push: bool,
    theme: &'static str,
}

fn demo() -> Slot<'static, Message> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("My Account"),
                menu::item("Profile").shortcut("⇧⌘P"),
                menu::item("Billing").shortcut("⌘B"),
                menu::item("Settings").shortcut("⌘S"),
                menu::separator(),
                menu::item("Team"),
                menu_sub![
                    "Invite users",
                    menu::item("Email"),
                    menu::item("Message"),
                    menu::separator(),
                    menu::item("More...")
                ],
                menu::item("New Team").shortcut("⌘+T"),
                menu::separator(),
                menu::item("GitHub"),
                menu::item("Support"),
                menu::item("API").disabled(true),
                menu::separator(),
                menu::item("Log out").shortcut("⇧⌘Q"),
            ]
        },
    )
    .align(dropdown_menu::Align::Start)
    .width(160)
    .into()
}

fn basic() -> Slot<'static, Message> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("My Account"),
                menu::item("Profile"),
                menu::item("Billing"),
                menu::item("Settings"),
                menu::separator(),
                menu::item("GitHub"),
                menu::item("Support"),
                menu::item("API").disabled(true),
            ]
        },
    )
    .into()
}

fn submenu() -> Slot<'static, Message> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::item("Team"),
                menu_sub![
                    "Invite users",
                    menu::item("Email"),
                    menu::item("Message"),
                    menu_sub![
                        "More options",
                        menu::item("Calendly"),
                        menu::item("Slack"),
                        menu::separator(),
                        menu::item("Webhook")
                    ],
                    menu::separator(),
                    menu::item("Advanced..."),
                ],
                menu::item("New Team").shortcut("⌘+T"),
            ]
        },
    )
    .into()
}

fn shortcuts() -> Slot<'static, Message> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("My Account"),
                menu::item("Profile").shortcut("⇧⌘P"),
                menu::item("Billing").shortcut("⌘B"),
                menu::item("Settings").shortcut("⌘S"),
                menu::separator(),
                menu::item("Log out").shortcut("⇧⌘Q"),
            ]
        },
    )
    .into()
}

fn icons() -> Slot<'static, Message> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                item(icon::user(), "Profile"),
                item(icon::credit_card(), "Billing"),
                item(icon::settings(), "Settings"),
                menu::separator(),
                item(icon::log_out(), "Log out").style(menu::Destructive),
            ]
        },
    )
    .into()
}

fn destructive() -> Slot<'static, Message> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Actions").style(button::Outline)),
        move |_| {
            menu_group![
                item(icon::pencil(), "Edit"),
                item(icon::share(), "Share"),
                menu::separator(),
                item(icon::trash_2(), "Delete").style(menu::Destructive),
            ]
        },
    )
    .into()
}

fn avatar_menu(photo: image::Handle) -> Slot<'static, Message> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        move |m| {
            m.trigger(
                button(avatar(Some(photo.clone())).fallback("LR"))
                    .square()
                    .rounded()
                    .translate(false)
                    .style(button::Ghost),
            )
        },
        move |_| {
            menu_group![
                item(icon::badge_check(), "Account"),
                item(icon::credit_card(), "Billing"),
                item(icon::bell(), "Notifications"),
                menu::separator(),
                item(icon::log_out(), "Sign Out"),
            ]
        },
    )
    .align(dropdown_menu::Align::End)
    .into()
}

fn checkboxes(checked: [bool; 3]) -> Slot<'static, [bool; 3]> {
    let toggle = move |i: usize, value| {
        let mut next = checked;
        next[i] = value;
        next
    };
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("Appearance"),
                menu::checkbox_item("Status Bar", checked[0], |v| toggle(0, v)),
                menu::checkbox_item("Activity Bar", checked[1], |v| toggle(1, v)).disabled(true),
                menu::checkbox_item("Panel", checked[2], |v| toggle(2, v)),
            ]
        },
    )
    .width(160)
    .into()
}

fn checkboxes_icons(checked: [bool; 3]) -> Slot<'static, [bool; 3]> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let toggle = move |i: usize, value| {
        let mut next = checked;
        next[i] = value;
        next
    };
    dropdown_menu(
        |m| m.trigger(button("Notifications").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("Notification Preferences"),
                menu::checkbox_item(
                    caption(icon::mail(), "Email notifications"),
                    checked[0],
                    |v| toggle(0, v)
                ),
                menu::checkbox_item(
                    caption(icon::message_square(), "SMS notifications"),
                    checked[1],
                    |v| toggle(1, v)
                ),
                menu::checkbox_item(
                    caption(icon::bell(), "Push notifications"),
                    checked[2],
                    |v| toggle(2, v)
                ),
            ]
        },
    )
    .width(192)
    .into()
}

fn radio_group(selected: &'static str) -> Slot<'static, &'static str> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("Panel Position"),
                menu::radio_item("Top", "top", Some(selected), |v| v),
                menu::radio_item("Bottom", "bottom", Some(selected), |v| v),
                menu::radio_item("Right", "right", Some(selected), |v| v),
            ]
        },
    )
    .width(128)
    .into()
}

fn radio_icons(selected: &'static str) -> Slot<'static, &'static str> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    dropdown_menu(
        |m| m.trigger(button("Payment Method").style(button::Outline)),
        move |_| {
            menu_group![
                menu::label("Select Payment Method"),
                menu::radio_item(
                    caption(icon::credit_card(), "Credit Card"),
                    "card",
                    Some(selected),
                    |v| v
                ),
                menu::radio_item(
                    caption(icon::wallet(), "PayPal"),
                    "paypal",
                    Some(selected),
                    |v| v
                ),
                menu::radio_item(
                    caption(icon::building_2(), "Bank Transfer"),
                    "bank",
                    Some(selected),
                    |v| v
                ),
            ]
        },
    )
    .width(224)
    .into()
}

fn complex(p: Preferences) -> Slot<'static, Preferences> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Complex Menu").style(button::Outline)),
        move |_| {
            let projects = menu_sub![
                caption(icon::ellipsis(), "More Projects"),
                item(icon::file_code(), "Project Gamma"),
                item(icon::file_code(), "Project Delta")
            ];
            let recent = menu_sub![
                caption(icon::folder_open(), "Open Recent"),
                menu::label("Recent Projects"),
                item(icon::file_code(), "Project Alpha"),
                item(icon::file_code(), "Project Beta"),
                projects,
                menu::separator(),
                item(icon::folder_search(), "Browse...")
            ];
            let theme = |g, label, value| {
                menu::radio_item(caption(g, label), value, Some(p.theme), |theme| {
                    Preferences { theme, ..p }
                })
            };
            let appearance = menu_sub![
                caption(icon::palette(), "Theme"),
                menu::label("Appearance"),
                theme(icon::sun(), "Light", "light"),
                theme(icon::moon(), "Dark", "dark"),
                theme(icon::monitor(), "System", "system")
            ];
            let notifications = menu_sub![
                caption(icon::bell(), "Notifications"),
                menu::label("Notification Types"),
                menu::checkbox_item(
                    caption(icon::bell(), "Push Notifications"),
                    p.push,
                    |push| Preferences { push, ..p }
                ),
                menu::checkbox_item(
                    caption(icon::mail(), "Email Notifications"),
                    p.email,
                    |email| Preferences { email, ..p }
                )
            ];
            let settings = menu_sub![
                caption(icon::settings(), "Settings"),
                menu::label("Preferences"),
                item(icon::keyboard(), "Keyboard Shortcuts"),
                item(icon::languages(), "Language"),
                notifications,
                menu::separator(),
                item(icon::shield(), "Privacy & Security")
            ];
            menu_group![
                menu::label("File"),
                item(icon::file(), "New File").shortcut("⌘N"),
                item(icon::folder(), "New Folder").shortcut("⇧⌘N"),
                recent,
                menu::separator(),
                item(icon::save(), "Save").shortcut("⌘S"),
                item(icon::download(), "Export").shortcut("⇧⌘E"),
                menu::separator(),
                menu::label("View"),
                menu::checkbox_item(caption(icon::eye(), "Show Sidebar"), p.email, |email| {
                    Preferences { email, ..p }
                }),
                menu::checkbox_item(
                    caption(icon::panels_top_left(), "Show Status Bar"),
                    p.sms,
                    |sms| Preferences { sms, ..p }
                ),
                appearance,
                menu::separator(),
                menu::label("Account"),
                item(icon::user(), "Profile").shortcut("⇧⌘P"),
                item(icon::credit_card(), "Billing"),
                settings,
                menu::separator(),
                item(icon::circle_question_mark(), "Help & Support"),
                item(icon::file_text(), "Documentation"),
                menu::separator(),
                item(icon::log_out(), "Sign Out")
                    .shortcut("⇧⌘Q")
                    .style(menu::Destructive),
            ]
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
            "demo" => demo().compose(),
            "basic" => basic().compose(),
            "submenu" => submenu().compose(),
            "shortcuts" => shortcuts().compose(),
            "icons" => icons().compose(),
            "destructive" => destructive().compose(),
            "avatar" => avatar_menu(alex_handle()).compose(),
            "checkboxes" => local([true, false, false], checkboxes),
            "checkboxes-icons" => local([true, false, true], checkboxes_icons),
            "radio-group" => local("bottom", radio_group),
            "radio-icons" => local("card", radio_icons),
            "complex" => local(
                Preferences {
                    email: true,
                    sms: false,
                    push: true,
                    theme: "light",
                },
                complex,
            ),
            _ => unreachable!("unknown dropdown menu preview id: {id}"),
        }
    }
}
