//! Message bubble styles, corners and composition.

use crate::page::rhea;
use crate::{Message, page};
use iced::border;
use iced::font::Weight;
use iced::widget::Id;
use iced::widget::text::LineHeight;
use iced::widget::{rich_text, svg, text::Span, themer};
use iced::{Element, Task};
use iced::{
    Fill,
    alignment::{
        Horizontal::{Left, Right},
        Vertical::{Bottom, Top},
    },
};
use iced::{Font, Widget};
use icedxdy::Size;
use icedxdy::theme::tweak::{Fg, Token};
use icedxdy::widget::bubble::{Destructive, Ghost, Muted, Outline, Secondary, Tinted};
use icedxdy::widget::{
    bubble, button, collapsible, column, container, popover, row, text, toast, tooltip,
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
    Block::Md("examples/browser/snippets/bubble-intro.md"),
    Block::Md("examples/browser/snippets/bubble-installation.md"),
    Block::Md("examples/browser/snippets/bubble-usage.md"),
    Block::Md("examples/browser/snippets/bubble-composition.md"),
    Block::Md("examples/browser/snippets/bubble-features.md"),
    Block::Example {
        id: "variants",
        title: "Variants",
        description: Some("Use `.style(...)` to change the visual treatment of a bubble."),
        snippet: "examples/browser/snippets/bubble-variants.md",
    },
    Block::Md("examples/browser/snippets/bubble-variants-guide.md"),
    Block::Example {
        id: "alignment",
        title: "Alignment",
        description: Some(
            "Use `.align(...)` to place a bubble on the left or right of the conversation.",
        ),
        snippet: "examples/browser/snippets/bubble-alignment.md",
    },
    Block::Md("examples/browser/snippets/bubble-alignment-guide.md"),
    Block::Example {
        id: "group-demo",
        title: "Bubble Group",
        description: Some(
            "Use `bubble::group(...)` for consecutive messages from the same sender. Set `.align(...)` on each bubble.",
        ),
        snippet: "examples/browser/snippets/bubble-group-demo.md",
    },
    Block::Example {
        id: "link-button",
        title: "Links and Buttons",
        description: Some(
            "Use `.interactive(true)` on a bubble to let it act as the trigger for an action.",
        ),
        snippet: "examples/browser/snippets/bubble-link-button.md",
    },
    Block::Md("examples/browser/snippets/bubble-link-button-guide.md"),
    Block::Example {
        id: "reactions",
        title: "Reactions",
        description: Some(
            "Pass text, icons, or buttons to `.reactions(...)`. Choose a horizontal and vertical alignment, and leave space between rows for reactions that overlap an edge.",
        ),
        snippet: "examples/browser/snippets/bubble-reactions.md",
    },
    Block::Example {
        id: "collapsible",
        title: "Show More / Collapsible",
        description: Some(
            "Use [Collapsible](icedxdy://component/collapsible) to let the reader expand a long message and collapse it again.",
        ),
        snippet: "examples/browser/snippets/bubble-collapsible.md",
    },
    Block::Example {
        id: "tooltip",
        title: "Tooltip",
        description: Some(
            "Wrap a bubble in a [Tooltip](icedxdy://component/tooltip) to reveal metadata on hover, such as when a message was read.",
        ),
        snippet: "examples/browser/snippets/bubble-tooltip.md",
    },
    Block::Example {
        id: "popover",
        title: "Popover",
        description: Some(
            "Pair a bubble with a [Popover](icedxdy://component/popover) to show the full error message for a failed action.",
        ),
        snippet: "examples/browser/snippets/bubble-popover.md",
    },
    Block::Group {
        title: "Accessibility",
        children: &[
            Block::Md("examples/browser/snippets/bubble-accessibility.md"),
            Block::Md("examples/browser/snippets/bubble-accessibility-reactions.md"),
            Block::Md("examples/browser/snippets/bubble-accessibility-interactive.md"),
            Block::Md("examples/browser/snippets/bubble-accessibility-color.md"),
        ],
    },
    Block::Group {
        title: "API Reference",
        children: &[
            Block::Md("examples/browser/snippets/bubble-api-reference.md"),
            Block::Md("examples/browser/snippets/bubble-api-content.md"),
            Block::Md("examples/browser/snippets/bubble-api-reactions.md"),
            Block::Md("examples/browser/snippets/bubble-api-group.md"),
        ],
    },
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
        bubble(label).style(Tinted).align(Right).interactive(true),
        toast::toast(notice),
    )
}

fn glyph(path: &'static str, muted: bool, destructive: bool) -> Slot<'static, Message> {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    );
    svg(svg::Handle::from_memory(source.into_bytes()))
        .width(16)
        .height(16)
        .style(move |theme: &Theme, _| svg::Style {
            color: Some(
                if destructive {
                    theme.destructive.base
                } else if muted {
                    theme.muted.foreground
                } else {
                    theme.neutral.foreground
                }
                .into(),
            ),
        })
        .boxed()
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
        let open = c.is_open();
        let chevron = if open {
            r#"<path d="m18 15-6-6-6 6"/>"#
        } else {
            r#"<path d="m6 9 6 6 6-6"/>"#
        };
        column![
            text(description),
            c.trigger(
                button(
                    row![
                        text(if open { "Show less" } else { "Show more" }),
                        glyph(chevron, true, false)
                    ]
                    .spacing(4)
                )
                .style(button::Link + Fg(Token::MutedForeground))
                .padding(0)
                .inset(0)
            )
        ]
        .into()
    })
    .into()
}
fn ghost_text(theme: &Theme) -> Slot<'static, Message> {
    let bold = Font::DEFAULT.weight(Weight::Bold);
    let code = Span::<()>::new("code")
        .font(Font::new("Geist Mono"))
        .size(12.25)
        .color(theme.neutral.foreground)
        .background(iced::Color::from(theme.muted.base))
        .border(border::rounded(4))
        .padding([1, 4]);
    column![
        rich_text([Span::<()>::new("Ghost bubbles work for assistant text, "), Span::<()>::new("markdown").font(bold), Span::<()>::new(", and other content that should not be framed.")])
            .size(14).line_height(LineHeight::Absolute(22.75.into())).boxed(),
        rich_text([Span::<()>::new("This is perfect for assistant messages that should not have a frame and can take the full width of the container. You can also render "), code, Span::<()>::new(" in it.")])
            .size(14).line_height(LineHeight::Absolute(22.75.into())).boxed(),
        text("Ghost bubbles are full width and can take the full width of the container.")
    ].spacing(16).into()
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
                .reactions(row!["👍", "🔥", "👀", "+2"].spacing(4), Right, Bottom)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn variants(theme: &Theme) -> Slot<'static, Message> {
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
            bubble(ghost_text(theme)).style(Ghost)
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
                suggestion("I forgot my password", "You clicked forgot password"),
                suggestion(
                    "I need help with my subscription",
                    "You clicked help with subscription"
                ),
                suggestion(
                    "Something else. Talk to a human.",
                    "You clicked something else. Talk to a human."
                ),
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
                .reactions(row!["👍", "😮"].spacing(4), Left, Bottom),
            bubble("Bold. Fine I'll add some tests. I'll let you know when they're done.")
                .style(Muted)
                .reactions(row!["👀", "🚀", "+2"].spacing(4), Right, Bottom),
            bubble("Tests passed on the first try. All 142 of them. Looking good!")
                .align(Right)
                .reactions(row!["🎉", "👏"].spacing(4), Left, Top),
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
                .reaction_padding(0)
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
                        button(glyph(r#"<path d="m20 6-11 11-5-5"/>"#, false, false))
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost),
                        "Read on Jan 5, 2026 at 4:32 PM"
                    ),
                    Right,
                    Bottom
                )
                .reaction_padding(0)
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
                            button(glyph(
                                r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/>"#,
                                false,
                                p.is_open()
                            ))
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost)
                        ),
                        |_| popover::header(
                            "Command failed with exit code 1",
                            "Could not open Cargo.lock: No such file or directory"
                        )
                    ),
                    Right,
                    Bottom
                )
                .reaction_padding(0)
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
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 0.0,
            padding: 40.into(),
            ..page::Backdrop::default()
        }
    }
    fn preview_theme(&self, outer: &Theme) -> Option<Theme> {
        Some(rhea::theme(outer))
    }
    fn themed_preview(&self, id: &str, outer: &Theme) -> Element<'_, Message, Theme> {
        let theme = rhea::theme(outer);
        let content = if id == "variants" {
            variants(&theme).into()
        } else {
            self.preview(id)
        };
        themer(Some(theme), content)
            .text_color(|theme| theme.neutral.foreground.into())
            .boxed()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        match id {
            "demo" => demo().into(),
            "variants" => variants(&Theme::light()).into(),
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
