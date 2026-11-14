use super::avatar::{alex_handle, evilrabbit_handle, maxleiter_handle};
use iced::widget::image;
use iced::{ContentFit, Fill, Shrink, Task, Widget};
use std::sync::OnceLock;

use icedcn::widget::{
    Column, Row, aspect_ratio, avatar, button, column, dropdown_menu, item, menu,
};
use icedcn::{Size, Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Item",
    description: "A versatile component that you can use to display any content.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/item-demo.md",
    },
    page::Block::Md("examples/browser/snippets/item-composition.md"),
    page::Block::Md("examples/browser/snippets/item-guide-item-vs-field.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "variant",
                title: "Variant",
                description: Some("Use `.style(..)` to change the visual style of the item."),
                snippet: "examples/browser/snippets/item-variant.md",
            },
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the item."),
                snippet: "examples/browser/snippets/item-size.md",
            },
            page::Block::Example {
                id: "icon",
                title: "Icon",
                description: Some("Use `item::media(..).style(item::Icon)` to display an icon."),
                snippet: "examples/browser/snippets/item-icon.md",
            },
            page::Block::Example {
                id: "avatar",
                title: "Avatar",
                description: Some("Use an avatar or avatar group as item media."),
                snippet: "examples/browser/snippets/item-avatar.md",
            },
            page::Block::Example {
                id: "image",
                title: "Image",
                description: Some("Display an image in an item."),
                snippet: "examples/browser/snippets/item-image.md",
            },
            page::Block::Example {
                id: "group",
                title: "Group",
                description: Some("Group related items together."),
                snippet: "examples/browser/snippets/item-group.md",
            },
            page::Block::Example {
                id: "header",
                title: "Header",
                description: None,
                snippet: "examples/browser/snippets/item-header.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link",
                description: Some("Make the whole row a keyboard-accessible link."),
                snippet: "examples/browser/snippets/item-link.md",
            },
            page::Block::Example {
                id: "dropdown",
                title: "Dropdown",
                description: None,
                snippet: "examples/browser/snippets/item-dropdown.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/item-api-reference.md"),
];

pub struct ItemPage {
    state: page::State,
}

impl ItemPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// An item with an inbox icon, a title and a description.
fn inbox<'a>(title: &'a str, description: &'a str) -> item::Item<'a, Message> {
    item::item([
        item::media(icon::inbox()).style(item::Icon).into(),
        item::content([
            item::title(title).into(),
            item::description(description).into(),
        ])
        .into(),
    ])
}

/// The `max-w-md` column the examples stack their items in.
fn stack<'a>(
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> iced::Element<'a, Message, Theme> {
    Column::with_children(items)
        .spacing(24)
        .width(Fill)
        .max_width(448.0)
        .into()
}

fn people() -> [(&'static str, &'static str, image::Handle); 3] {
    [
        ("Alex Chen", "alex@example.com", alex_handle()),
        ("maxleiter", "maxleiter@example.com", maxleiter_handle()),
        ("evilrabbit", "evilrabbit@example.com", evilrabbit_handle()),
    ]
}

fn artwork(bytes: &'static [u8], handle: &'static OnceLock<image::Handle>) -> image::Image {
    image(
        handle
            .get_or_init(|| image::Handle::from_bytes(bytes))
            .clone(),
    )
    .content_fit(ContentFit::Cover)
    .width(Fill)
    .height(Fill)
    .border_radius(6)
}

impl page::Page for ItemPage {
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
        let top = Message::Page(self.page_index(), page::Message::LinkClicked("#".into()));
        match id {
            "demo" => stack([
                item::item([
                    item::content([
                        item::title("Basic Item").into(),
                        item::description("A simple item with title and description.").into(),
                    ])
                    .into(),
                    item::actions([button("Action")
                        .style(button::Outline)
                        .size(Size::Sm)
                        .into()]),
                ])
                .style(item::Outline)
                .into(),
                item::item([
                    item::media(icon::badge_check().size(20)).into(),
                    item::content([item::title("Your profile has been verified.").into()]).into(),
                    item::actions([icon::chevron_right().size(16).into()]),
                ])
                .style(item::Outline)
                .size(item::Size::Sm)
                .on_press(top.clone()),
            ]),

            "variant" => stack([
                inbox("Default Variant", "Transparent background with no border.").into(),
                inbox("Outline Variant", "Outlined style with a visible border.")
                    .style(item::Outline)
                    .into(),
                inbox("Muted Variant", "Muted background for secondary content.")
                    .style(item::Muted)
                    .into(),
            ]),

            "size" => stack([
                inbox("Default Size", "The standard size for most use cases.")
                    .style(item::Outline)
                    .into(),
                inbox("Small Size", "A compact size for dense layouts.")
                    .style(item::Outline)
                    .size(item::Size::Sm)
                    .into(),
                inbox("Extra Small Size", "The most compact size available.")
                    .style(item::Outline)
                    .size(item::Size::Xs)
                    .into(),
            ]),

            "icon" => column![
                item::item([
                    item::media(icon::shield_alert()).style(item::Icon).into(),
                    item::content([
                        item::title("Security Alert").into(),
                        item::description("New login detected from unknown device.").into(),
                    ])
                    .into(),
                    item::actions([button("Review")
                        .size(Size::Sm)
                        .style(button::Outline)
                        .into()]),
                ])
                .style(item::Outline)
            ]
            .width(Fill)
            .max_width(512.0)
            .into(),

            "avatar" => stack([
                item::item([
                    item::media(
                        avatar(Some(evilrabbit_handle()))
                            .fallback("ER")
                            .size(avatar::Lg),
                    )
                    .into(),
                    item::content([
                        item::title("Evil Rabbit").into(),
                        item::description("Last seen 5 months ago").into(),
                    ])
                    .into(),
                    item::actions([button(icon::plus())
                        .rounded()
                        .square()
                        .size(Size::Sm)
                        .style(button::Outline)
                        .into()]),
                ])
                .style(item::Outline)
                .into(),
                item::item([
                    item::media(avatar::group([
                        avatar(Some(alex_handle())).fallback("AC"),
                        avatar(Some(maxleiter_handle())).fallback("LR"),
                        avatar(Some(evilrabbit_handle())).fallback("ER"),
                    ]))
                    .into(),
                    item::content([
                        item::title("No Team Members").into(),
                        item::description("Invite your team to collaborate on this project.")
                            .into(),
                    ])
                    .into(),
                    item::actions([button("Invite")
                        .style(button::Outline)
                        .size(Size::Sm)
                        .into()]),
                ])
                .style(item::Outline)
                .into(),
            ])
            .into(),

            "image" => {
                let rows = [
                    (
                        "Midnight City Lights",
                        "Electric Nights",
                        "Neon Dreams",
                        "3:45",
                        include_bytes!("../../assets/item/midnight-city-lights.jpg").as_slice(),
                    ),
                    (
                        "Coffee Shop Conversations",
                        "Urban Stories",
                        "The Morning Brew",
                        "4:05",
                        include_bytes!("../../assets/item/coffee-shop-conversations.jpg")
                            .as_slice(),
                    ),
                    (
                        "Digital Rain",
                        "Binary Beats",
                        "Cyber Symphony",
                        "3:30",
                        include_bytes!("../../assets/item/digital-rain.jpg").as_slice(),
                    ),
                ];
                static COVERS: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
                column(rows.into_iter().zip(&COVERS).map(
                    |((title, album, artist, duration, cover), cache)| {
                        item::item([
                            item::media(artwork(cover, cache).boxed())
                                .style(item::Image)
                                .into(),
                            item::content([
                                item::title(format!("{title} - {album}")).into(),
                                item::description(artist).into(),
                            ])
                            .into(),
                            item::content([item::description(duration).into()])
                                .width(Shrink)
                                .into(),
                        ])
                        .style(item::Outline)
                        .on_press(top.clone())
                    },
                ))
                .spacing(16)
                .width(Fill)
                .max_width(448.0)
            }
            .into(),

            "group" => column(people().map(|(name, email, portrait)| {
                item::item([
                    item::media(avatar(Some(portrait)).fallback(&name[..1])).into(),
                    item::content([item::title(name).into(), item::description(email).into()])
                        .spacing(4)
                        .into(),
                    item::actions([button(icon::plus())
                        .style(button::Ghost)
                        .square()
                        .rounded()
                        .into()]),
                ])
                .style(item::Outline)
                .into()
            }))
            .max_width(384.0)
            .width(Fill)
            .into(),

            "header" => {
                let models = [
                    (
                        "v0-1.5-sm",
                        "Everyday tasks and UI generation.",
                        include_bytes!("../../assets/item/v0-small.jpg").as_slice(),
                    ),
                    (
                        "v0-1.5-lg",
                        "Advanced thinking or reasoning.",
                        include_bytes!("../../assets/item/v0-large.jpg").as_slice(),
                    ),
                    (
                        "v0-2.0-mini",
                        "Open Source model for everyone.",
                        include_bytes!("../../assets/item/v0-mini.jpg").as_slice(),
                    ),
                ];
                static IMAGES: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
                Row::with_children(models.into_iter().zip(&IMAGES).map(
                    |((name, description, asset), cache)| {
                        item::item([
                            item::header([aspect_ratio(artwork(asset, cache).boxed(), 1.0).into()]),
                            item::content([
                                item::title(name).into(),
                                item::description(description).into(),
                            ])
                            .into(),
                        ])
                        .orientation(icedcn::Vertical)
                        .style(item::Outline)
                        .into()
                    },
                ))
                .spacing(16)
                .width(Fill)
            }
            .into(),

            "link" => column![
                item::item([
                    item::content([
                        item::title("Visit our documentation").into(),
                        item::description("Learn how to get started with our components.").into()
                    ])
                    .into(),
                    item::actions([icon::chevron_right().size(16).into()])
                ])
                .on_press(top),
                item::item([
                    item::content([
                        item::title("External resource").into(),
                        item::description("Opens the project in your browser.").into()
                    ])
                    .into(),
                    item::actions([icon::external_link().size(16).into()])
                ])
                .style(item::Outline)
                .on_press(Message::OpenUrl("https://github.com/airstrike/icedcn"))
            ]
            .spacing(16)
            .width(Fill)
            .max_width(448.0)
            .into(),

            "dropdown" => dropdown_menu(
                |m| m.trigger(button!["Select", icon::chevron_down()].style(button::Outline)),
                |_| {
                    menu::group(people().map(|(name, email, portrait)| {
                        menu::item(
                            item::item([
                                item::media(avatar(Some(portrait)).fallback(&name[..1]).size(26.0))
                                    .into(),
                                item::content([
                                    item::title(name).into(),
                                    item::description(email).into(),
                                ])
                                .spacing(0)
                                .into(),
                            ])
                            .size(item::Size::Xs),
                        )
                        .into()
                    }))
                },
            )
            .width(192)
            .align(dropdown_menu::Align::End)
            .into(),

            _ => unreachable!("unknown item preview id: {id}"),
        }
    }
}
