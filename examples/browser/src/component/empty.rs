use super::avatar::{alex_handle, evilrabbit_handle, maxleiter_handle};
use iced::mouse::Cursor;
use iced::widget::{canvas, stack};
use iced::{Element, Fill, Point, Rectangle, Task, Widget};
use icedcn::theme::container::Style;
use icedcn::widget::input_group::Align;

use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::{avatar, button, column, container, empty, input, input_group, kbd, row};
use icedcn::{Size, Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Empty",
    description: "Use the Empty component to display an empty state.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/empty-demo.md",
    },
    page::Block::Md("examples/browser/snippets/empty-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "card",
                title: "Card",
                description: Some("Content with a row of actions and a link below."),
                snippet: "examples/browser/snippets/empty-card.md",
            },
            page::Block::Example {
                id: "outline",
                title: "Outline",
                description: Some("Frame the empty state with a dashed border."),
                snippet: "examples/browser/snippets/empty-outline.md",
            },
            page::Block::Example {
                id: "background",
                title: "Background",
                description: Some("Use a muted background to distinguish an empty state."),
                snippet: "examples/browser/snippets/empty-background.md",
            },
            page::Block::Example {
                id: "avatar",
                title: "Avatar",
                description: Some("Use an avatar in the empty state media."),
                snippet: "examples/browser/snippets/empty-avatar.md",
            },
            page::Block::Example {
                id: "avatar-group",
                title: "Avatar Group",
                description: Some("Show a group of avatars in the media slot."),
                snippet: "examples/browser/snippets/empty-avatar-group.md",
            },
            page::Block::Example {
                id: "input-group",
                title: "InputGroup",
                description: Some("Use an input group to search from an empty state."),
                snippet: "examples/browser/snippets/empty-input-group.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/empty-api-reference.md"),
];

pub struct EmptyPage {
    state: page::State,
}

impl EmptyPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// muted "Learn More ↗" link button.
fn learn_more<'a>(label: &'a str, size: Size) -> Slot<'a, Message> {
    button![Slot::from(label), icon::arrow_up_right()]
        .style(button::Link + Fg(Token::MutedForeground))
        .size(size)
        .into()
}

struct Outline;
impl canvas::Program<Message, Theme> for Outline {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _: Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let path = canvas::Path::rounded_rectangle(
            Point::new(0.5, 0.5),
            iced::Size::new(
                (bounds.width - 1.0).max(0.0),
                (bounds.height - 1.0).max(0.0),
            ),
            12.0.into(),
        );
        let stroke = canvas::Stroke {
            line_dash: canvas::LineDash {
                segments: &[4.0, 3.0],
                offset: 0,
            },
            ..canvas::Stroke::default()
                .with_color(theme.neutral.border.into())
                .with_width(1.0)
        };
        frame.stroke(&path, stroke);
        vec![frame.into_geometry()]
    }
}

impl page::Page for EmptyPage {
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
            "demo" => empty::empty()
                .media(empty::media(icon::folder_code()).style(empty::Icon))
                .title("No Projects Yet")
                .description(
                    "You haven't created any projects yet. Get started by creating your \
                     first project.",
                )
                .content(
                    column![
                        row![
                            button("Create Project"),
                            button("Import Project").style(button::Outline),
                        ]
                        .spacing(8),
                        learn_more("Learn More", Size::Sm),
                    ]
                    .spacing(16)
                    .align_x(iced::Center),
                )
                .into(),

            "card" => empty::empty()
                .media(empty::media(icon::folder()).style(empty::Icon))
                .title("No projects yet")
                .description(
                    "You haven't created any projects yet. Get started by creating your \
                     first project.",
                )
                .content(
                    column![
                        row![
                            button("Create project"),
                            button("Import project").style(button::Outline),
                        ]
                        .spacing(8),
                        learn_more("Learn more", Size::Default),
                    ]
                    .spacing(16)
                    .align_x(iced::Center),
                )
                .into(),

            "outline" => stack![
    Element::from(empty::empty().media(empty::media(icon::cloud()).style(empty::Icon)).title("Cloud Storage Empty")
        .description("Upload files to your cloud storage to access them anywhere.")
        .content(button("Upload Files").style(button::Outline).size(Size::Sm))),
    canvas(Outline).width(Fill).height(Fill)
].boxed(),

            "background" => container(empty::empty().media(empty::media(icon::bell()).style(empty::Icon)).title("No Notifications")
    .description("You're all caught up. New notifications will appear here.")
    .content(button![icon::refresh_ccw(), "Refresh"].style(button::Outline))).width(Fill)
    .style(|theme: &Theme| Style { background: Some(theme.muted.base.alpha(0.3).into()), ..Style::default() }).into(),

            "avatar" => empty::empty().media(empty::media(avatar(Some(alex_handle())).fallback("AC").size(48.0)))
    .title("User Offline").description("This user is currently offline. You can leave a message to notify them or try again later.")
    .content(button("Leave Message").size(Size::Sm)).into(),

            "avatar-group" => empty::empty().media(empty::media(avatar::group([
    avatar(Some(alex_handle())).fallback("AC").size(48.0), avatar(Some(maxleiter_handle())).fallback("LR").size(48.0), avatar(Some(evilrabbit_handle())).fallback("ER").size(48.0)])))
    .title("No Team Members").description("Invite your team to collaborate on this project.")
    .content(button![icon::plus(), "Invite Members"].size(Size::Sm)).into(),

            "input-group" => empty::empty().title("404 - Not Found")
    .description("The page you're looking for doesn't exist. Try searching for what you need below.")
    .content(column![
        input_group(input("Try searching for pages...")).addon(input_group::addon(icon::search())).addon(input_group::addon(kbd("/")).align(Align::InlineEnd)),
        row![empty::description("Need help?"), button("Contact support").style(button::Link).on_press(Message::Page(self.page_index(), page::Message::LinkClicked("#".into())))]
        .align_y(iced::Center)
    ].spacing(16).align_x(iced::Center).max_width(288.0)).into(),

            _ => unreachable!("unknown empty preview id: {id}"),
        }
    }
}
