// RTL is omitted under the browser component scope.

use std::sync::OnceLock;

use iced::Task;
use iced::widget::{Svg, image, svg};

use icedxdy::theme::avatar::badge_style;
use icedxdy::widget::{avatar, button, column, dropdown_menu, menu, row};
use icedxdy::{Color, Theme, theme};

use crate::Message;
use crate::page;

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
        description: Some("Use `.size(avatar::Sm)` or `.size(avatar::Lg)` to change the avatar size."),
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
                avatar(Some(alex_handle())).fallback("AC"),
                avatar::badge(
                    avatar(Some(evilrabbit_handle())).fallback("ER"),
                    avatar::dot(),
                )
                .badge_style(presence),
                avatar::group([
                    avatar(Some(alex_handle())).fallback("AC"),
                    avatar(Some(maxleiter_handle())).fallback("LR"),
                    avatar(Some(evilrabbit_handle())).fallback("ER"),
                ])
                .count("+3"),
            ]
            .spacing(if self.wide { 48 } else { 24 })
            .wrap()
            .align_y(iced::Center)
            .into(),

            "basic" => avatar(Some(alex_handle())).fallback("AC").into(),

            "badge" => avatar::badge(
                avatar(Some(alex_handle())).fallback("AC"),
                avatar::dot(),
            )
            .badge_style(presence)
            .into(),

            "badge-icon" => avatar::badge(
                avatar(Some(pranathip_handle())).fallback("PP"),
                plus(8.0).style(|theme: &Theme, _| svg::Style {
                    color: Some(theme.primary.foreground.into()),
                }),
            )
            .into(),

            "group" => avatar::group([
                avatar(Some(alex_handle())).fallback("AC"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .into(),

            "group-count" => avatar::group([
                avatar(Some(alex_handle())).fallback("AC"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .count("+3")
            .into(),

            "group-count-icon" => avatar::group([
                avatar(Some(alex_handle())).fallback("AC"),
                avatar(Some(maxleiter_handle())).fallback("LR"),
                avatar(Some(evilrabbit_handle())).fallback("ER"),
            ])
            .count(plus(16.0).style(|theme: &Theme, _| svg::Style {
                color: Some(theme.muted.foreground.into()),
            }))
            .into(),

            "size" => row![
                avatar(Some(alex_handle()))
                    .fallback("AC")
                    .size(avatar::Sm),
                avatar(Some(alex_handle())).fallback("AC"),
                avatar(Some(alex_handle()))
                    .fallback("AC")
                    .size(avatar::Lg),
            ]
            .spacing(8)
            .wrap()
            .align_y(iced::Center)
            .into(),

            "dropdown" => dropdown_menu(
                |m| {
                    m.trigger(
                        button(avatar(Some(alex_handle())).fallback("AC"))
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



fn presence(theme: &Theme) -> theme::container::Style {
    let mut style = badge_style(theme);
    let color = if theme.neutral.base.l < 0.5 {
        iced::Color::from_rgb8(1, 102, 48)
    } else {
        iced::Color::from_rgb8(0, 166, 62)
    };
    style.background = Some(Color::from_srgb(color).into());
    style.border.width = 0.0;
    style
}

fn plus(size: f32) -> Svg<'static, Theme> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5v14"/></svg>"#))
        .width(size)
        .height(size)
}
