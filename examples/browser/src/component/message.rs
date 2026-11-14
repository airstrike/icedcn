use crate::icon;
use crate::{Message, page};
use iced::alignment::{Horizontal::Right, Vertical::Bottom};
use iced::widget::Id;
use iced::widget::component::{Component, component};
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use iced::{Element, Task};
use icedcn::theme::text::Foreground;
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::bubble::Muted;
use icedcn::widget::status_marker::shimmer;
use icedcn::widget::{
    attachment, avatar, bubble, button, column, container, message, row, space, status_marker,
    text, toggle, tooltip,
};
use icedcn::{Size, Weight};
use icedcn::{Slot, Theme};
use page::Block;
use std::sync::LazyLock;

pub static META: page::Meta = page::Meta {
    title: "Message",
    description: "Displays a message in a conversation, with optional avatar, header, footer, and alignment.",
};
pub static BLOCKS: &[Block] = &[
    Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/message-demo.md",
    },
    Block::Md("examples/browser/snippets/message-usage.md"),
    Block::Md("examples/browser/snippets/message-composition.md"),
    Block::Example {
        id: "avatar",
        title: "Avatar",
        description: None,
        snippet: "examples/browser/snippets/message-avatar.md",
    },
    Block::Example {
        id: "group",
        title: "Group",
        description: None,
        snippet: "examples/browser/snippets/message-group.md",
    },
    Block::Example {
        id: "header-footer",
        title: "Header and Footer",
        description: None,
        snippet: "examples/browser/snippets/message-header-footer.md",
    },
    Block::Example {
        id: "actions",
        title: "Actions",
        description: None,
        snippet: "examples/browser/snippets/message-actions.md",
    },
    Block::Example {
        id: "attachment",
        title: "Attachment",
        description: None,
        snippet: "examples/browser/snippets/message-attachment.md",
    },
    Block::Md("examples/browser/snippets/message-api-reference.md"),
];
pub struct MessagePage {
    state: page::State,
}
impl MessagePage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
        })
    }
}

fn frame(body: impl Into<Slot<'static, Message>>) -> Slot<'static, Message> {
    container(body)
        .width(Fill)
        .max_width(384)
        .padding([48, 0])
        .into()
}
fn portrait(number: usize, fallback: &'static str) -> Slot<'static, Message> {
    static AVATARS: LazyLock<[image::Handle; 3]> = LazyLock::new(|| {
        [
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/02.png").as_slice()),
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/03.png").as_slice()),
            image::Handle::from_bytes(include_bytes!("../../assets/avatars/10.png").as_slice()),
        ]
    });
    avatar(Some(AVATARS[number].clone()))
        .fallback(fallback)
        .into()
}

fn demo_example() -> Slot<'static, Message> {
    frame(
        column![
            message(bubble("Deploying to prod real quick."))
                .align(Right)
                .avatar(portrait(2, "ME")),
            message(bubble("It's 4:55 PM. On a Friday.").style(Muted)).avatar(portrait(0, "R")),
            message(bubble("It's a one-line change."))
                .align(Right)
                .avatar(portrait(2, "ME"))
                .footer("Delivered"),
            message(bubble::group([
                bubble("It's always a one-line change 😭.")
                    .style(Muted)
                    .into(),
                bubble("Alright, let me take a look.")
                    .style(Muted)
                    .reactions("👍", Right, Bottom)
                    .into()
            ]))
            .avatar(portrait(0, "R")),
            status_marker(shimmer("Oliver is typing..."))
        ]
        .spacing(24)
        .width(Fill),
    )
}

fn avatar_example() -> Slot<'static, Message> {
    frame(column![
message(bubble("The build failed during dependency installation.").style(Muted)).avatar(portrait(1, "R")),
message(bubble("Can you share the exact error?")).align(Right).avatar(portrait(2, "R")),
message(bubble::group([bubble("Here's the error from the logs").style(Muted).into(),
bubble("Something went wrong with the build. The libraries are not installed correctly. Try running the build again.").style(Muted).into()])).avatar(portrait(1, "R"))
].spacing(24))
}

fn group_example() -> Slot<'static, Message> {
    frame(message::group([
        message(bubble("I checked the registry addresses.").style(Muted))
            .avatar(space().width(32).height(32))
            .into(),
        message(
            bubble("The component and example JSON now live under the UI registry.").style(Muted),
        )
        .avatar(portrait(0, "AC"))
        .into(),
    ]))
}

fn header_footer_example() -> Slot<'static, Message> {
    frame(
        column![
            message(bubble("I already checked the logs.").style(Muted)).header("Olivia"),
            message(bubble(
                "Send the report to the team. Ping @alex if you need help."
            ))
            .align(Right)
            .footer(
                row![
                    text("Read").size(12.).medium(),
                    text("Yesterday").size(12.).weight(Weight::Normal)
                ]
                .spacing(4)
            )
        ]
        .spacing(32)
        .width(Fill),
    )
}

const INSTALL: &str = "The install failure is coming from the workspace package.";
#[derive(Default)]
struct FeedbackState {
    vote: Option<bool>,
    delivered: bool,
}
#[derive(Clone, Debug)]
enum Action {
    Copy,
    Vote(bool),
    Retry,
}
struct Actions;
impl<'a> Component<'a, Message, Theme> for Actions {
    type State = FeedbackState;
    type Event = Action;
    fn update(
        &self,
        state: &mut FeedbackState,
        action: Action,
        _: &iced::Renderer,
    ) -> Option<Message> {
        match action {
            Action::Copy => return Some(Message::CopyText(INSTALL.to_owned())),
            Action::Vote(up) => {
                state.vote = if state.vote == Some(up) {
                    None
                } else {
                    Some(up)
                }
            }
            Action::Retry => state.delivered = true,
        }
        None
    }
    fn view(&self, state: &FeedbackState) -> impl Widget<Action, Theme> + 'a {
        let actions = row![
            tooltip(
                button(icon::copy())
                    .square()
                    .style(button::Ghost)
                    .on_press(Action::Copy),
                "Copy"
            ),
            tooltip(
                toggle(icon::thumbs_up())
                    .pressed(state.vote == Some(true))
                    .on_pressed_change(|_| Action::Vote(true)),
                "Like"
            ),
            tooltip(
                toggle(icon::thumbs_down())
                    .pressed(state.vote == Some(false))
                    .on_pressed_change(|_| Action::Vote(false)),
                "Dislike"
            )
        ];
        let delivery: Slot<'a, Action> = if state.delivered {
            text("Delivered").size(12.).into()
        } else {
            row![
                text("Failed to send")
                    .size(12.)
                    .style(Foreground + Fg(Token::Destructive)),
                tooltip(
                    button(icon::refresh_ccw())
                        .square()
                        .size(Size::Xs)
                        .style(button::Ghost)
                        .on_press(Action::Retry),
                    "Retry"
                )
            ]
            .spacing(8)
            .into()
        };
        Element::from(
            column![
                message(bubble(INSTALL).style(Muted)).footer(actions),
                message(bubble("Okay drop me a link. Taking a look..."))
                    .align(Right)
                    .footer(delivery)
            ]
            .spacing(32),
        )
    }
}
fn actions_example() -> Slot<'static, Message> {
    frame(component(Actions).boxed())
}
fn attachment_example() -> Slot<'static, Message> {
    static WORKSPACE: LazyLock<image::Handle> = LazyLock::new(|| {
        image::Handle::from_bytes(
            include_bytes!("../../assets/attachments/workspace.jpg").as_slice(),
        )
    });
    let cover = image(WORKSPACE.clone())
        .width(80)
        .height(80)
        .content_fit(ContentFit::Cover)
        .border_radius(8);
    let download = tooltip(
        attachment::action(icon::download())
            .size(Size::Sm)
            .style(button::Secondary)
            .on_press(Message::SaveAttachment),
        "Download",
    );
    frame(
        column![
            message(
                column![
                    attachment("").vertical(true).width(96).media(cover.boxed()),
                    bubble(
                        "Here's the image. Can you add it to the PDF? Use it for the cover page."
                    )
                ]
                .align_x(Right)
                .spacing(10)
            )
            .align(Right),
            message(
                column![
                    bubble("Done. Here's the PDF with the image added as the cover page.")
                        .style(Muted),
                    attachment("sales-dashboard.pdf")
                        .width(250)
                        .description("PDF · 2.4 MB")
                        .media(icon::file_text())
                        .action(download)
                ]
                .spacing(10)
            ),
            message(bubble("Thanks. Looks good.")).align(Right)
        ]
        .spacing(32)
        .width(Fill),
    )
}

impl page::Page for MessagePage {
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
    fn scrollable_id(&self) -> Id {
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
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        match id {
            "demo" => demo_example().into(),
            "avatar" => avatar_example().into(),
            "group" => group_example().into(),
            "header-footer" => header_footer_example().into(),
            "actions" => actions_example().into(),
            "attachment" => attachment_example().into(),
            _ => unreachable!("unknown message preview {id}"),
        }
    }
}
