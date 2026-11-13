use crate::font::GEIST_MONO_NAME;
use crate::icon;
use crate::{Message, page};
use iced::Font;
use iced::widget::Id;
use iced::{Element, Task};
use iced::{
    Fill,
    alignment::{
        Horizontal::{Left, Right},
        Vertical::{Bottom, Top},
    },
};
use icedxdy::Size;
use icedxdy::widget::bubble::{Destructive, Ghost, Muted, Outline, Secondary, Tinted};
use icedxdy::widget::{
    bubble, button, collapsible, column, container, popover, rich_text, span, text, toast, tooltip,
};
use icedxdy::{Slot, Theme};
use page::Block;

pub static META: page::Meta = page::Meta {
    title: "Bubble",
    description: "Displays conversational content in a message bubble. Supports variants, alignment, grouping, reactions, and collapsible content.",
};
pub static BLOCKS: &[Block] = &[
    Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/bubble-demo.md",
    },
    Block::Md("examples/browser/snippets/bubble-usage.md"),
    Block::Md("examples/browser/snippets/bubble-composition.md"),
    Block::Example {
        id: "variants",
        title: "Variants",
        description: None,
        snippet: "examples/browser/snippets/bubble-variants.md",
    },
    Block::Example {
        id: "alignment",
        title: "Alignment",
        description: None,
        snippet: "examples/browser/snippets/bubble-alignment.md",
    },
    Block::Example {
        id: "group-demo",
        title: "Bubble Group",
        description: None,
        snippet: "examples/browser/snippets/bubble-group-demo.md",
    },
    Block::Example {
        id: "link-button",
        title: "Links and Buttons",
        description: None,
        snippet: "examples/browser/snippets/bubble-link-button.md",
    },
    Block::Example {
        id: "reactions",
        title: "Reactions",
        description: None,
        snippet: "examples/browser/snippets/bubble-reactions.md",
    },
    Block::Example {
        id: "collapsible",
        title: "Show More / Collapsible",
        description: None,
        snippet: "examples/browser/snippets/bubble-collapsible.md",
    },
    Block::Example {
        id: "tooltip",
        title: "Tooltip",
        description: None,
        snippet: "examples/browser/snippets/bubble-tooltip.md",
    },
    Block::Example {
        id: "popover",
        title: "Popover",
        description: None,
        snippet: "examples/browser/snippets/bubble-popover.md",
    },
    Block::Md("examples/browser/snippets/bubble-api-reference.md"),
];
pub struct BubblePage {
    state: page::State,
}
impl BubblePage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
        })
    }
}

fn frame(body: impl Into<Slot<'static, Message>>) -> Slot<'static, Message> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
fn suggestion(label: &'static str, notice: &'static str) -> Slot<'static, Message> {
    toast::trigger(
        button(label).style(button::Ghost).inset(0),
        toast::toast(notice),
    )
    .into()
}
fn review() -> Slot<'static, Message> {
    const REVIEW: &str = "The accessibility review found two focus states that were visually too subtle in dark mode.\n\nI checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface.\n\nThe dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.\n\nI also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.";
    collapsible(|c| {
        let description = if c.is_open() {
            REVIEW.into()
        } else {
            format!("{}...", &REVIEW[..180])
        };
        column![
            text(description),
            c.trigger(
                button(if c.is_open() {
                    "Show less  ⌃"
                } else {
                    "Show more  ⌄"
                })
                .style(button::Link)
                .inset(0)
            )
        ]
        .spacing(4)
        .into()
    })
    .into()
}
fn ghost_text() -> Slot<'static, Message> {
    column![
        rich_text([span("Ghost bubbles work for assistant text, "),span("markdown").bold(),span(", and other content that should not be framed.")]),
        rich_text([span("This is perfect for assistant messages that should not have a frame and can take the full width of the container. You can also render "),span("code").font(Font::new(GEIST_MONO_NAME)),span(" in it.")]),
        text("Ghost bubbles are full width and can take the full width of the container.")
    ].spacing(12).into()
}
fn demo() -> Slot<'static, Message> {
    frame(
        column![
            bubble("Hey there! what's up?").align(Right),
            bubble::group([
                bubble("Hey! Want to see chat bubbles?").style(Muted).into(),
                bubble(
                    "I can group messages, switch sides, and keep the whole thread easy to scan."
                )
                .style(Muted)
                .reactions("👍", Right, Bottom)
                .into()
            ]),
            bubble("Sure. Hit me with your best demo.").align(Right),
            bubble("Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand.")
                .style(Muted)
                .reactions("👍 🔥 👀 +2", Right, Bottom)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn variants() -> Slot<'static, Message> {
    frame(
        column![
            bubble("This is the default primary bubble."),
            bubble("This is the secondary variant.")
                .style(Secondary)
                .align(Right),
            bubble("This one is muted. It uses a lower emphasis color for the chat bubble.")
                .style(Muted)
                .reactions("👍", Right, Bottom),
            bubble(
                "This one is tinted. The tint is a softer color derived from the primary color."
            )
            .style(Tinted)
            .align(Right),
            bubble("We can also use an outlined variant.").style(Outline),
            bubble("Or a destructive variant with a reaction.")
                .style(Destructive)
                .align(Right)
                .reactions("🔥", Right, Bottom),
            bubble(ghost_text()).style(Ghost)
        ]
        .spacing(48)
        .width(Fill),
    )
}

fn alignment() -> Slot<'static, Message> {
    frame(
        column![
            bubble("This bubble is aligned to the start. This is the default alignment.")
                .style(Muted),
            bubble("This bubble is aligned to the end. Use this for user messages.").align(Right)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn group_demo() -> Slot<'static, Message> {
    frame(
        column![
            bubble("Can you tell me what's the issue?").style(Muted),
            bubble::group([
                bubble("You tell me!").align(Right).into(),
                bubble("It worked yesterday. You broke it!")
                    .align(Right)
                    .into(),
                bubble("Find the bug and fix it.")
                    .align(Right)
                    .reactions("👀", Left, Bottom)
                    .into()
            ]),
            bubble("Want me to diff yesterday's you against today's you? It's a bit embarrassing.")
                .style(Muted)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn link_button() -> Slot<'static, Message> {
    frame(
        column![
            bubble("How can I help you today?").style(Muted),
            bubble::group([
                bubble(suggestion(
                    "I forgot my password",
                    "You clicked forgot password"
                ))
                .style(Tinted)
                .align(Right)
                .into(),
                bubble(suggestion(
                    "I need help with my subscription",
                    "You clicked help with subscription"
                ))
                .style(Tinted)
                .align(Right)
                .into(),
                bubble(suggestion(
                    "Something else. Talk to a human.",
                    "You clicked something else. Talk to a human."
                ))
                .style(Tinted)
                .align(Right)
                .into()
            ])
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn reactions() -> Slot<'static, Message> {
    frame(
        column![
            bubble("I don't need tests, I know my code works.")
                .style(Muted)
                .align(Right)
                .reactions("👍 😮", Left, Bottom),
            bubble("Bold. Fine I'll add some tests. I'll let you know when they're done.")
                .style(Muted)
                .reactions("👀 🚀 +2", Right, Bottom),
            bubble("Tests passed on the first try. All 142 of them. Looking good!")
                .align(Right)
                .reactions("🎉 👏", Left, Top),
            bubble("Are you sure I can run this command?")
                .style(Destructive)
                .reactions(
                    toast::trigger(
                        button("Yes, run it").size(Size::Xs).style(button::Ghost),
                        toast::toast("You clicked yes, running command...")
                            .kind(toast::Kind::Success)
                    ),
                    Right,
                    Bottom
                )
        ]
        .spacing(48)
        .width(Fill),
    )
}

fn collapsible_example() -> Slot<'static, Message> {
    frame(
        column![
            bubble("How can I help you today?").style(Muted),
            bubble(review()).style(Muted).align(Right)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn tooltip_example() -> Slot<'static, Message> {
    frame(
        column![
            bubble("Did you remove the stale route?").style(Secondary),
            bubble("Yes, removed it from the registry.")
                .align(Right)
                .reactions(
                    tooltip(
                        button(icon::check())
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost),
                        "Read on Jan 5, 2026 at 4:32 PM"
                    ),
                    Right,
                    Bottom
                )
        ]
        .spacing(16)
        .width(Fill),
    )
}

fn popover_example() -> Slot<'static, Message> {
    frame(
        column![
            bubble("Run the build script.").align(Right),
            bubble("Failed to run the command.")
                .style(Destructive)
                .reactions(
                    popover(
                        |p| p.trigger(
                            button(icon::info())
                                .size(Size::Xs)
                                .square()
                                .style(button::Ghost)
                        ),
                        |_| popover::header(
                            "Command failed with exit code 1",
                            "ENOENT: no such file or directory, open pnpm-lock.yaml"
                        )
                    ),
                    Right,
                    Bottom
                )
        ]
        .spacing(16)
        .width(Fill),
    )
}

impl page::Page for BubblePage {
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
            "demo" => demo().into(),
            "variants" => variants().into(),
            "alignment" => alignment().into(),
            "group-demo" => group_demo().into(),
            "link-button" => link_button().into(),
            "reactions" => reactions().into(),
            "collapsible" => collapsible_example().into(),
            "tooltip" => tooltip_example().into(),
            "popover" => popover_example().into(),
            _ => unreachable!("unknown bubble preview {id}"),
        }
    }
}
