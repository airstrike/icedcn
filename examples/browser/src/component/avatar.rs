use std::sync::OnceLock;

use iced::Task;
use iced::widget::image;

use icedxdy::Theme;
use icedxdy::widget::{avatar, button, column, dropdown_menu, menu, row};

use crate::Message;
use crate::icon;
use crate::page;

pub(crate) fn shadcn_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/shadcn.png").as_slice())
        })
        .clone()
}

pub(crate) fn maxleiter_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(
                include_bytes!("../../assets/avatars/maxleiter.png").as_slice(),
            )
        })
        .clone()
}

pub(crate) fn evilrabbit_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(
                include_bytes!("../../assets/avatars/evilrabbit.png").as_slice(),
            )
        })
        .clone()
}

pub(crate) fn pranathip_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(
                include_bytes!("../../assets/avatars/pranathip.png").as_slice(),
            )
        })
        .clone()
}

pub static META: page::Meta = page::Meta {
    title: "Avatar",
    description: "An image element with a fallback for representing the user.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/avatar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/avatar-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A basic avatar component with an image and a fallback."),
                snippet: "examples/browser/snippets/avatar-basic.md",
            },
            page::Block::Example {
                id: "badge",
                title: "Badge",
                description: Some(
                    "Use `avatar::badge(avatar, badge)` to place a badge at the bottom right of the avatar.",
                ),
                snippet: "examples/browser/snippets/avatar-badge.md",
            },
            page::Block::Example {
                id: "badge-icon",
                title: "Badge with Icon",
                description: Some("Pass an icon as the second argument to `avatar::badge`."),
                snippet: "examples/browser/snippets/avatar-badge-icon.md",
            },
            page::Block::Example {
                id: "group",
                title: "Avatar Group",
                description: Some("Pass the avatars to `avatar::group(...)`."),
                snippet: "examples/browser/snippets/avatar-group.md",
            },
            page::Block::Example {
                id: "group-count",
                title: "Avatar Group Count",
                description: Some("Use `.count(...)` to show how many additional avatars the group represents."),
                snippet: "examples/browser/snippets/avatar-group-count.md",
            },
            page::Block::Example {
                id: "group-count-icon",
                title: "Avatar Group with Icon",
                description: Some("Pass an icon to `.count(...)` to show it at the end of the group."),
                snippet: "examples/browser/snippets/avatar-group-count-icon.md",
            },
            page::Block::Example {
                id: "size",
                title: "Sizes",
                description: Some("Use `.size(...)` to change the avatar size."),
                snippet: "examples/browser/snippets/avatar-size.md",
            },
            page::Block::Example {
                id: "dropdown",
                title: "Dropdown",
                description: Some("Use an avatar as a dropdown menu trigger."),
                snippet: "examples/browser/snippets/avatar-dropdown.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/avatar-api-reference.md"),
];

pub struct AvatarPage {
    state: page::State,
}

impl AvatarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for AvatarPage {
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
            // Hero (avatar-demo.tsx): three avatars in a row at gap-6 (24px),
            // demonstrating the basic avatar, an avatar with a status badge,
            // and a group with a +3 count.
            "demo" => row![
                avatar(Some(shadcn_handle())).fallback("CN"),
                avatar::badge(
                    avatar(Some(evilrabbit_handle())).fallback("ER"),
                    avatar::dot(),
                ),
                avatar::group([
                    avatar(Some(shadcn_handle())).fallback("CN"),
                    avatar(Some(maxleiter_handle())).fallback("LR"),
                    avatar(Some(evilrabbit_handle())).fallback("ER"),
                ])
                .count("+3"),
            ]
            .spacing(24)
            .align_y(iced::Center)
            .into(),

            // Basic (avatar-basic.tsx): single avatar — image + fallback.
            "basic" => avatar(Some(shadcn_handle())).fallback("CN").into(),

            // Badge (avatar-badge.tsx): avatar + corner status dot.
            "badge" => {
                avatar::badge(avatar(Some(shadcn_handle())).fallback("CN"), avatar::dot()).into()
            }

            // Badge with Icon (avatar-badge-icon.tsx): avatar + corner badge
            // containing a `+` icon (lucide PlusIcon).
            "badge-icon" => avatar::badge(
                avatar(Some(pranathip_handle())).fallback("PP"),
                icon::plus(),
            )
            .into(),

            // Avatar Group (avatar-group.tsx): three overlapping avatars.
            "group" => avatar::group([
                avatar(Some(shadcn_handle())).fallback("CN"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .into(),

            // Avatar Group Count (avatar-group-count.tsx): three avatars +
            // a `+3` text count chip on the right.
            "group-count" => avatar::group([
                avatar(Some(shadcn_handle())).fallback("CN"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .count("+3")
            .into(),

            // Avatar Group with Icon (avatar-group-count-icon.tsx): three
            // avatars + a `+` icon count chip on the right.
            "group-count-icon" => avatar::group([
                avatar(Some(shadcn_handle())).fallback("CN"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .count(icon::plus())
            .into(),

            // Sizes (avatar-size.tsx): three avatars at sm / default / lg in
            // a row at gap-2 (8px). Same persona at three sizes mirrors the
            // shadcn example (which uses the same avatar three times).
            "size" => row![
                avatar(Some(shadcn_handle()))
                    .fallback("CN")
                    .size(avatar::Sm),
                avatar(Some(shadcn_handle())).fallback("CN"),
                avatar(Some(shadcn_handle()))
                    .fallback("CN")
                    .size(avatar::Lg),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            "dropdown" => dropdown_menu(
                |m| {
                    m.trigger(
                        button(avatar(Some(shadcn_handle())).fallback("CN"))
                            .style(button::Ghost)
                            .square()
                            .rounded(),
                    )
                },
                |_| {
                    column![
                        menu::item("Profile"),
                        menu::item("Billing"),
                        menu::item("Settings"),
                        menu::separator(),
                        menu::item("Log out").style(menu::Destructive)
                    ]
                    .width(iced::Fill)
                    .into()
                },
            )
            .width(128)
            .into(),

            _ => unreachable!("unknown avatar preview id: {id}"),
        }
    }
}
