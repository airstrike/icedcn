// RTL is omitted under the browser component scope.
// These examples keep the original Andy Terra, Héctor Ramón and iced logo fixtures.

use std::sync::OnceLock;

use iced::Task;
use iced::widget::image;

use icedcn::Theme;
use icedcn::widget::avatar::badge;
use icedcn::widget::{avatar, button, column, dropdown_menu, menu, row};

use crate::Message;
use crate::{icon, page};

pub(crate) fn alex_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/02.png").as_slice())
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

fn airstrike_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(
                include_bytes!("../../assets/avatars/airstrike.png").as_slice(),
            )
        })
        .clone()
}

fn hecrj_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/hecrj.png").as_slice())
        })
        .clone()
}

fn iced_handle() -> image::Handle {
    static IMAGE: OnceLock<image::Handle> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/iced.png").as_slice())
        })
        .clone()
}

pub static META: page::Meta = page::Meta {
    title: "Avatar",
    description: "An image with a text fallback for a user’s avatar.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/avatar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/avatar-installation.md"),
    page::Block::Md("examples/browser/snippets/avatar-usage.md"),
    page::Block::Md("examples/browser/snippets/avatar-composition.md"),
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
            "Place a status dot at the lower right with `avatar::badge(avatar, avatar::dot())`.",
        ),
        snippet: "examples/browser/snippets/avatar-badge.md",
    },
    page::Block::Md("examples/browser/snippets/avatar-badge-customization.md"),
    page::Block::Example {
        id: "badge-icon",
        title: "Badge with Icon",
        description: Some("Pass an icon as the second argument to `avatar::badge(...)`."),
        snippet: "examples/browser/snippets/avatar-badge-icon.md",
    },
    page::Block::Example {
        id: "group",
        title: "Avatar Group",
        description: Some("Pass the avatars to `avatar::group(...)` to overlap them."),
        snippet: "examples/browser/snippets/avatar-group.md",
    },
    page::Block::Example {
        id: "group-count",
        title: "Avatar Group Count",
        description: Some("Use `.count(\"+3\")` to show how many more people belong to the group."),
        snippet: "examples/browser/snippets/avatar-group-count.md",
    },
    page::Block::Example {
        id: "group-count-icon",
        title: "Avatar Group with Icon",
        description: Some("Pass an icon to the avatar group’s `.count(...)` builder."),
        snippet: "examples/browser/snippets/avatar-group-count-icon.md",
    },
    page::Block::Example {
        id: "size",
        title: "Sizes",
        description: Some(
            "Use `.size(avatar::Sm)` or `.size(avatar::Lg)` to change the avatar size.",
        ),
        snippet: "examples/browser/snippets/avatar-size.md",
    },
    page::Block::Example {
        id: "dropdown",
        title: "Dropdown",
        description: Some(concat!(
            "You can use the `Avatar` component ",
            "as a trigger for a dropdown menu.",
        )),
        snippet: "examples/browser/snippets/avatar-dropdown.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/avatar-api-reference.md"),
            page::Block::Md("examples/browser/snippets/avatar-api-image.md"),
            page::Block::Md("examples/browser/snippets/avatar-api-fallback.md"),
            page::Block::Md("examples/browser/snippets/avatar-api-badge.md"),
            page::Block::Md("examples/browser/snippets/avatar-api-group.md"),
            page::Block::Md("examples/browser/snippets/avatar-api-group-count.md"),
        ],
    },
];

pub struct AvatarPage {
    state: page::State,
    wide: bool,
}

impl AvatarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
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

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 768.0;
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // The viewport sets the gap. The available content width controls wrapping.
            "demo" => row![
                avatar(airstrike_handle()).fallback("AT"),
                avatar::badge(avatar(hecrj_handle()).fallback("HR"), avatar::dot(),)
                    .badge_style(badge::presence),
                avatar::group![
                    avatar(airstrike_handle()).fallback("AT"),
                    avatar(hecrj_handle()).fallback("HR"),
                    avatar(iced_handle()).fallback("IC"),
                ]
                .count("+3"),
            ]
            .spacing(if self.wide { 48 } else { 24 })
            .wrap()
            .align_y(iced::Center)
            .into(),

            "basic" => avatar(airstrike_handle()).fallback("AT").into(),

            "badge" => avatar::badge(avatar(hecrj_handle()).fallback("HR"), avatar::dot())
                .badge_style(badge::presence)
                .into(),

            "badge-icon" => avatar::badge(
                avatar(iced_handle()).fallback("IC"),
                icon::plus().size(8).line_height(1.0),
            )
            .into(),

            "group" => avatar::group![
                avatar(airstrike_handle()).fallback("AT"),
                avatar(hecrj_handle()).fallback("HR"),
                avatar(iced_handle()).fallback("IC"),
            ]
            .into(),

            "group-count" => avatar::group![
                avatar(airstrike_handle()).fallback("AT"),
                avatar(hecrj_handle()).fallback("HR"),
                avatar(iced_handle()).fallback("IC"),
            ]
            .count("+3")
            .into(),

            "group-count-icon" => avatar::group![
                avatar(airstrike_handle()).fallback("AT"),
                avatar(hecrj_handle()).fallback("HR"),
                avatar(iced_handle()).fallback("IC"),
            ]
            .count(icon::plus().size(16).line_height(1.0))
            .into(),

            "size" => row![
                avatar(airstrike_handle()).fallback("AT").size(avatar::Sm),
                avatar(airstrike_handle()).fallback("AT"),
                avatar(airstrike_handle()).fallback("AT").size(avatar::Lg),
            ]
            .spacing(8)
            .wrap()
            .align_y(iced::Center)
            .into(),

            "dropdown" => dropdown_menu(
                |m| {
                    m.trigger(
                        button(avatar(airstrike_handle()).fallback("AT"))
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
