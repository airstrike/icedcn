use crate::icon;
use crate::{Message, page};
use iced::Alignment::Center;
use iced::Task;
use iced::alignment::{
    Horizontal::{Left, Right},
    Vertical::Bottom,
};
use iced::widget::component::{Component, component};
use iced::widget::{button::StyleFn, float, stack};
use iced::{Element, Fill, Widget};
use icedxdy::Theme;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::message_scroller::{Command, Position, Reader, Target};
use icedxdy::widget::{
    bubble, button, column, container, dropdown_menu, hover_card, menu, message, message_scroller,
    row, select, separator, slider, space, status_marker, tabs, text, toggle_group,
};
use icedxdy::{Size, Slot, card};
use std::time::Duration;

pub static META: page::Meta = page::Meta {
    title: "Message Scroller",
    description: "A chat scroll container that anchors turns, opens saved transcripts, follows streamed responses, loads history without jumping, and jumps to any message.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/message-scroller-demo.md",
    },
    page::Block::Md("examples/browser/snippets/message-scroller-experience.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-overview.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-composition.md"),
    page::Block::Group {
        title: "Core Concepts",
        children: &[
            page::Block::Example {
                id: "anchoring",
                title: "Anchoring Turns",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-anchoring.md",
            },
            page::Block::Example {
                id: "group-chat",
                title: "Group Chat",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-group-chat.md",
            },
            page::Block::Example {
                id: "previous-context",
                title: "Keeping Context Visible",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-previous-context.md",
            },
            page::Block::Example {
                id: "streaming",
                title: "Following the Live Edge",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-streaming.md",
            },
            page::Block::Example {
                id: "opening-position",
                title: "Opening Saved Threads",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-opening-position.md",
            },
            page::Block::Md("examples/browser/snippets/message-scroller-initial-layout.md"),
            page::Block::Example {
                id: "load-history",
                title: "Loading Earlier Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-load-history.md",
            },
            page::Block::Example {
                id: "animation",
                title: "Animating New Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-animation.md",
            },
            page::Block::Example {
                id: "commands",
                title: "Jumping to Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-commands.md",
            },
            page::Block::Example {
                id: "visibility",
                title: "Tracking the Reader's Position",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-visibility.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Reading Scroll State",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-scrollable.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/message-scroller-performance.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-virtualization.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-accessibility.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-api-reference.md"),
];
mod fixtures;
mod tick;
use fixtures::{Role, Turn};
#[derive(Clone, Copy, PartialEq, Eq)]
enum Demo {
    Chat,
    Anchoring,
    Group,
    Context,
    Streaming,
    Opening,
    History,
    Animation,
    Commands,
    Visibility,
    Scrollable,
}
impl Demo {
    fn turns(self) -> &'static [Turn] {
        match self {
            Self::Chat => fixtures::DEMO,
            Self::Anchoring => fixtures::ANCHORING,
            Self::Group => fixtures::GROUP_CHAT,
            Self::Context => fixtures::PREVIOUS_CONTEXT,
            Self::Streaming => fixtures::STREAMING,
            Self::Opening => fixtures::OPENING_POSITION,
            Self::History => fixtures::LOAD_HISTORY,
            Self::Animation => fixtures::ANIMATION,
            Self::Commands => fixtures::COMMANDS,
            Self::Visibility => fixtures::VISIBILITY,
            Self::Scrollable => fixtures::SCROLLABLE,
        }
    }
    fn info(self) -> (&'static str, &'static str) {
        match self {
            Self::Chat => ("New Chat", "How can I help you today?"),
            Self::Anchoring => (
                "Anchoring Turns",
                "Choose which role settles near the top edge.",
            ),
            Self::Group => (
                "Group Chat",
                "A group chat with several participants and an assistant. The Marker is marked as a turn.",
            ),
            Self::Context => (
                "Keeping Context Visible",
                "New turns keep part of the previous reply in view.",
            ),
            Self::Streaming => (
                "Streaming Messages",
                "Auto-scroll follows the live edge of the conversation.",
            ),
            Self::Opening => ("Opening Position", "Choose where a saved transcript opens."),
            Self::History => ("Load History", "Prepended messages keep your place."),
            Self::Animation => (
                "Animation",
                "Choose how user messages are animated when they are added to the conversation.",
            ),
            Self::Commands => ("Commands", "Drive the transcript from outside."),
            Self::Visibility => ("Transcript Outline", "Track the current anchored turn."),
            Self::Scrollable => (
                "Scroll Status",
                "Where the reader can go scroll to based on current scroll position.",
            ),
        }
    }
    fn caption(self) -> &'static str {
        match self {
            Self::Chat => "Demo is read only. Press send to send messages.",
            Self::Anchoring => {
                "Toggle the anchor role, then send messages to compare where turns settle."
            }
            Self::Group => {
                "When a user joins, a marker is created. scrollAnchor on the marker marks it as the next turn"
            }
            Self::Context => "Adjust the slider and send. Observe the previous message peak",
            Self::Streaming => "Streaming is simulated. autoScroll is enabled.",
            Self::Opening => {
                "Toggle the defaultScrollPosition to see where the transcript starts when you open the thread"
            }
            Self::History => "Click Load History to load the entire conversation",
            Self::Animation => "Select an animation then click send to see it in action.",
            Self::Commands => "Use the controls to jump to any message in the conversation.",
            Self::Visibility => "Open the outline to jump between anchored turns as you read.",
            Self::Scrollable => "Scroll the transcript to see the footer update.",
        }
    }
    fn initial(self) -> Model {
        let total = self.turns().len();
        let count = match self {
            Self::Chat | Self::Anchoring | Self::Streaming | Self::Animation => 0,
            Self::Group => 3,
            Self::Context => 2,
            _ => total,
        };
        Model {
            initialized: true,
            count,
            first: if self == Self::History {
                total.saturating_sub(5)
            } else {
                0
            },
            peek: if self == Self::Context { 64 } else { 0 },
            ..Model::default()
        }
    }
    fn scripted(self) -> bool {
        matches!(
            self,
            Self::Chat | Self::Streaming | Self::Context | Self::Animation
        )
    }
    fn anchor(self, state: &Model, i: usize) -> bool {
        let row = self.turns()[i];
        match self {
            Self::Group => i == 2 || row.role == Role::Marker,
            Self::History => false,
            Self::Anchoring => {
                row.role
                    == if state.assistant_anchor {
                        Role::Assistant
                    } else {
                        Role::User
                    }
            }
            _ => row.role == Role::User,
        }
    }
}
#[derive(Clone, Debug)]
enum Event {
    Send,
    Reset,
    Tick,
    Reader(Reader),
    Peek(f32),
    Anchor(String),
    Open(String),
    Preset(String),
    Jump(Target),
}
#[derive(Default)]
struct Model {
    initialized: bool,
    first: usize,
    count: usize,
    peek: u16,
    assistant_anchor: bool,
    preset: usize,
    opening: Position,
    command: Option<Command>,
    reader: Reader,
    stream: Option<usize>,
    entrance: Option<(usize, u8)>,
}
impl Model {
    fn jump(&mut self, target: Target) {
        let sequence = self.command.map_or(1, |cmd| cmd.sequence.wrapping_add(1));
        self.command = Some(Command { sequence, target });
    }
}
const PRESETS: [&str; 7] = [
    "Fade",
    "Slide Up",
    "Slide Side",
    "Pop",
    "Spring Bounce",
    "Blur Fade",
    "Scale Fade",
];
impl<'a> Component<'a, page::Message, Theme> for Demo {
    type State = Model;
    type Event = Event;
    fn diff(&mut self, state: &mut Model) {
        if !state.initialized {
            *state = self.initial();
        }
    }
    fn update(&self, state: &mut Model, event: Event, _: &iced::Renderer) -> Option<page::Message> {
        let turns = self.turns();
        match event {
            Event::Reset => *state = self.initial(),
            Event::Send if *self == Demo::History => {
                state.first = 0;
                return Some(page::Message::Notice(
                    "History loaded".into(),
                    "Scroll up to see earlier messages.".into(),
                ));
            }
            Event::Send if state.count < turns.len() && state.stream.is_none() => {
                let index = state.count;
                state.count += 1;
                if *self == Demo::Animation {
                    state.entrance = Some((index, 0));
                }
                if self.scripted()
                    && state.count < turns.len()
                    && turns[state.count].role == Role::Assistant
                {
                    state.count += 1;
                    state.stream = Some(0);
                }
            }
            Event::Tick => {
                if let Some(n) = state.stream {
                    let bytes = turns[state.count - 1].text;
                    let mut end = (n + 12).min(bytes.len());
                    while end < bytes.len() && !bytes.is_char_boundary(end) {
                        end += 1;
                    }
                    state.stream = (end < bytes.len()).then_some(end);
                }
                if let Some((i, n)) = state.entrance {
                    state.entrance = (n < 12).then_some((i, n + 1));
                }
            }
            Event::Reader(reader) => state.reader = reader,
            Event::Peek(px) => state.peek = px as u16,
            Event::Anchor(role) => {
                *state = self.initial();
                state.assistant_anchor = role == "Assistant";
            }
            Event::Preset(p) => state.preset = PRESETS.iter().position(|&n| n == p).unwrap_or(0),
            Event::Open(pos) => {
                state.opening = match pos.as_str() {
                    "start" => Position::Start,
                    "end" => Position::End,
                    _ => Position::LastAnchor,
                };
                state.jump(match state.opening {
                    Position::Start => Target::Start,
                    Position::End => Target::End,
                    Position::LastAnchor => Target::Item(2),
                });
            }
            Event::Jump(target) => state.jump(target),
            Event::Send => {}
        }
        None
    }
    fn view(&self, state: &Model) -> impl Widget<Event, Theme> + 'a {
        self.frame(state)
    }
}
impl Demo {
    fn frame(self, state: &Model) -> Element<'static, Event, Theme> {
        let (title, description) = self.info();
        let mut header = card::header(title).description(description);
        if !matches!(
            self,
            Self::Opening | Self::Commands | Self::Visibility | Self::Scrollable
        ) {
            let initial = self.initial();
            header = header.action(
                button(icon::refresh_ccw())
                    .size(Size::Sm)
                    .square()
                    .style(button::Outline)
                    .disabled(
                        (state.count == initial.count && state.first == initial.first)
                            || (self == Self::Animation && state.stream.is_some()),
                    )
                    .on_press(Event::Reset),
            );
        }
        if self == Self::Commands {
            header = header.action(self.controls(state));
        }
        let panel: Slot<'static, Event> = if state.count == 0 {
            let (title, desc) = match self {
                Self::Chat => (
                    "Morning, shadcn!",
                    "What are we working on today? Press send to start a new conversation",
                ),
                Self::Anchoring => (
                    "No anchored messages yet",
                    "Send the first message to see the selected role anchor.",
                ),
                Self::Streaming => (
                    "Ready to Stream",
                    "Press send to stream a scripted launch summary.",
                ),
                _ => (
                    "No Messages Yet",
                    "Click the button below to send the first message.",
                ),
            };
            container(
                column![
                    icon::message_square().size(26),
                    text::large(title).semibold(),
                    text::muted(desc)
                ]
                .spacing(8)
                .align_x(Center),
            )
            .center_x(Fill)
            .center_y(Fill)
            .padding(12)
            .into()
        } else {
            let items = (state.first..state.count).map(|i| {
                let turn = self.turns()[i];
                let value = if i + 1 == state.count {
                    state.stream.map_or(turn.text, |n| &turn.text[..n])
                } else {
                    turn.text
                };
                let animate = state
                    .entrance
                    .filter(|(index, _)| *index == i)
                    .map(|(_, frame)| frame);
                let body = self.turn(turn, value, state.preset, animate);
                message_scroller::item(i as u64, body).anchor(self.anchor(state, i))
            });
            let history_end = (self == Self::History).then(|| {
                message_scroller::item(
                    u64::MAX,
                    status_marker("End of Conversation").style(status_marker::Separator),
                )
            });
            let transcript = message_scroller(items.chain(history_end))
                .height(Fill)
                .auto_scroll(self == Self::Streaming)
                .scroll_margin(match self {
                    Self::Context => 24.0,
                    Self::Opening => 64.0,
                    _ => 12.0,
                })
                .previous_item_peek(f32::from(state.peek))
                .default_scroll_position(if self == Self::Commands {
                    Position::End
                } else if self == Self::Scrollable {
                    Position::Start
                } else {
                    state.opening
                })
                .smooth_commands(matches!(self, Self::Commands | Self::Visibility))
                .command(state.command)
                .on_reader_change(Event::Reader);
            let mut layers = vec![Element::from(transcript)];
            if state.reader.can_scroll_down {
                layers.push(
                    container(
                        button(icon::arrow_down())
                            .style(button::Outline)
                            .rounded()
                            .size(Size::Sm)
                            .square()
                            .on_press(Event::Jump(Target::End)),
                    )
                    .center_x(Fill)
                    .align_y(Bottom)
                    .height(Fill)
                    .padding(8)
                    .into(),
                );
            }
            stack(layers).width(Fill).height(Fill).boxed().into()
        };
        let mut body = column![
            container(header).width(Fill).padding([12, 0]),
            separator(),
            panel,
        ]
        .spacing(0)
        .width(Fill)
        .height(560);
        if !matches!(self, Self::Commands | Self::Visibility) {
            body = body.push(separator().into()).push(
                container(card::footer(self.controls(state)))
                    .padding([10, 0])
                    .into(),
            );
        }
        // Keep the transcript at the same tree path when the last chunk lands.
        let mut layers = vec![body.into()];
        if state.stream.is_some() || state.entrance.is_some() {
            layers.push(tick::Tick.boxed());
        }
        let body = stack(layers).width(Fill).height(560).boxed();
        let card = card(body).width(Fill).boxed();
        let card: Slot<'static, Event> = if self == Self::Visibility {
            let outline = column(
                self.turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.role == Role::User)
                    .map(|(i, t)| {
                        button(text::small(short(t.text)))
                            .size(Size::Sm)
                            .style(if state.reader.current_anchor == Some(i as u64) {
                                ButtonStyle::from(button::Secondary)
                            } else {
                                ButtonStyle::from(button::Ghost)
                            })
                            .on_press(Event::Jump(Target::Item(i as u64)))
                            .into()
                    }),
            )
            .spacing(4)
            .width(Fill);
            let ticks = column(
                self.turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.role == Role::User)
                    .map(|(i, _)| {
                        let current = state.reader.current_anchor == Some(i as u64);
                        container(space::horizontal())
                            .width(16)
                            .height(2)
                            .style(move |theme| Style {
                                background: Some(
                                    if current {
                                        theme.neutral.foreground
                                    } else {
                                        theme.muted.foreground.alpha(0.4)
                                    }
                                    .into(),
                                ),
                                ..Style::default()
                            })
                            .into()
                    }),
            )
            .spacing(4);
            let trigger = button(ticks).square().style(button::Ghost);
            let nav = hover_card(trigger, outline)
                .side(hover_card::Side::Left)
                .open_delay(Duration::ZERO)
                .close_delay(Duration::ZERO)
                .width(256);
            row![card, nav].spacing(4).align_y(Center).into()
        } else {
            card.into()
        };
        column![
            card,
            text::muted(self.caption())
                .size(12)
                .width(Fill)
                .align_x(Center)
        ]
        .spacing(16)
        .width(if self == Self::Visibility { 420 } else { 384 })
        .into()
    }
    fn turn(
        self,
        turn: Turn,
        value: &'static str,
        preset: usize,
        frame: Option<u8>,
    ) -> Slot<'static, Event> {
        if turn.role == Role::Marker {
            return status_marker(text::small(value))
                .style(status_marker::Separator)
                .into();
        }
        let user = turn.role == Role::User;
        let alignment = if user { Right } else { Left };
        let mut content: Slot<'static, Event> = bubble(text::small(value))
            .style(match turn.role {
                Role::User => bubble::Muted,
                Role::Guest => bubble::Tinted,
                _ => bubble::Ghost,
            })
            .align(alignment)
            .into();
        if let Some(frame) = frame {
            content = entrance(content, preset, frame);
        }
        let mut turn_view = message(content).align(alignment);
        if !user && !turn.speaker.is_empty() {
            turn_view = turn_view.header(text::xs(turn.speaker).semibold());
        }
        turn_view.into()
    }
    fn controls(self, state: &Model) -> Slot<'static, Event> {
        let next = self.turns().get(state.count);
        let disabled = next.is_none() || state.stream.is_some();
        let send = button(icon::arrow_up())
            .size(Size::Sm)
            .square()
            .disabled(disabled)
            .on_press(Event::Send);
        match self {
            Self::Chat | Self::Context | Self::Streaming => {
                let plus = dropdown_menu(
                    |m| {
                        m.trigger(
                            button(icon::plus())
                                .size(Size::Sm)
                                .square()
                                .style(button::Outline),
                        )
                    },
                    |_| {
                        column![
                            menu::item(row![icon::file_plus(), "Add Photos & Files"].spacing(8)),
                            menu::separator(),
                            menu::item(row![icon::image(), "Create Image"].spacing(8)),
                            menu::item(row![icon::search(), "Deep Research"].spacing(8)),
                            menu::item(row![icon::globe(), "Web Search"].spacing(8))
                        ]
                        .width(Fill)
                        .into()
                    },
                )
                .width(176)
                .side(dropdown_menu::Side::Top);
                let prompt = next.map_or("No messages queued. Reset the conversation.", |t| t.text);
                let mut line = row![plus].spacing(8).align_y(Center).width(Fill);
                if self == Self::Context {
                    line = line
                        .push(text::xs(format!("{}px", state.peek)).into())
                        .push(
                            container(
                                slider(64.0..=128.0)
                                    .value(f32::from(state.peek))
                                    .disabled(state.stream.is_some())
                                    .on_value_change(Event::Peek),
                            )
                            .width(80)
                            .into(),
                        );
                }
                let line = line.push(space::horizontal().into()).push(send.into());
                let input = container(text::small(prompt))
                    .height(44)
                    .width(Fill)
                    .clip(true);
                container(column![input, line].spacing(4))
                    .padding(8)
                    .width(Fill)
                    .style(|theme| Style {
                        border: Border {
                            color: theme.input,
                            width: 1.,
                            radius: 8.0.into(),
                        },
                        ..Style::default()
                    })
                    .into()
            }
            Self::Anchoring => {
                let value = if state.assistant_anchor {
                    "Assistant"
                } else {
                    "User"
                };
                row![
                    toggle_group(|| ["User", "Assistant"].map(|s| toggle_group::item(s, s)))
                        .value([value])
                        .on_value_change(move |v| Event::Anchor(
                            v.first().cloned().unwrap_or_else(|| value.to_owned())
                        )),
                    space::horizontal(),
                    send
                ]
                .width(Fill)
                .into()
            }
            Self::Animation => row![
                options(&PRESETS, PRESETS[state.preset], Event::Preset),
                space::horizontal(),
                send
            ]
            .width(Fill)
            .into(),
            Self::Opening => {
                let current = match state.opening {
                    Position::Start => "start",
                    Position::End => "end",
                    Position::LastAnchor => "last-anchor",
                };
                container(
                    tabs(|t| t.list(["start", "end", "last-anchor"].map(|s| t.trigger(s, s))))
                        .value(current)
                        .on_value_change(Event::Open),
                )
                .center_x(Fill)
                .into()
            }
            Self::Group => column![
                button(if state.count < 4 {
                    "Add Rocky"
                } else {
                    "Send Message as Rocky"
                })
                .style(button::Secondary)
                .width(Fill)
                .disabled(disabled)
                .on_press(Event::Send),
                text::muted(if state.count < 4 {
                    "This will create a marker and make it the anchor"
                } else {
                    "Now send Rocky's reply into the conversation"
                })
                .size(12)
            ]
            .spacing(8)
            .align_x(Center)
            .into(),
            Self::History => column![
                container(
                    button(if state.first == 0 {
                        "History Loaded"
                    } else {
                        "Load History"
                    })
                    .style(button::Secondary)
                    .width(Fill)
                    .disabled(state.first == 0)
                    .on_press(Event::Send)
                )
                .id("transcript-load-history")
                .width(Fill),
                text::xs("Restore earlier messages while keeping your place.")
            ]
            .align_x(Center)
            .spacing(6)
            .into(),
            Self::Commands => dropdown_menu(
                |m| m.trigger(button("Jump to...").style(button::Secondary)),
                move |_| {
                    column(
                        std::iter::once(menu::label("Conversations")).chain(
                            self.turns()
                                .iter()
                                .enumerate()
                                .filter(|(_, t)| t.role == Role::User)
                                .map(|(i, t)| {
                                    menu::item(text::small(short(t.text)))
                                        .on_select(Event::Jump(Target::Item(i as u64)))
                                        .into()
                                }),
                        ),
                    )
                    .width(Fill)
                    .into()
                },
            )
            .width(256)
            .into(),
            Self::Visibility => text::muted("Track the current anchored turn.")
                .size(12.0)
                .into(),
            Self::Scrollable => text::muted(
                match (state.reader.can_scroll_up, state.reader.can_scroll_down) {
                    (true, true) => "You can scroll both ways.",
                    (false, true) => "You are at the top. You can only scroll down.",
                    (true, false) => "You are at the bottom. You can only scroll up.",
                    (false, false) => "All messages fit in the viewport.",
                },
            )
            .size(14.0)
            .into(),
        }
    }
}
fn short(source: &str) -> String {
    let mut chars = source.chars();
    let mut text: String = chars.by_ref().take(39).collect();
    if chars.take(4).count() == 4 {
        text.push_str("...");
        text
    } else {
        source.to_owned()
    }
}
fn options(
    values: &'static [&'static str],
    selected: &str,
    callback: fn(String) -> Event,
) -> Slot<'static, Event> {
    select(values.iter().map(|&s| select::item(s, s)))
        .position(select::Position::Popper)
        .value(selected)
        .on_value_change(callback)
        .icon(icon::chevron_down)
        .width(140)
        .into()
}
fn entrance(content: Slot<'static, Event>, preset: usize, frame: u8) -> Slot<'static, Event> {
    let t = (1.0 - f32::from(frame) / 12.0).clamp(0.0, 1.0);
    let progress = t * t;
    let (x, y, scale) = match preset {
        1 => (0.0, 10.0 * progress, 1.0),
        2 => (18.0 * progress, 0.0, 1.0),
        3 => (0.0, 6.0 * progress, 1.0 - 0.06 * progress),
        4 => (0.0, (t * 9.0).sin() * 9.0 * t, 1.0 - 0.08 * progress),
        5 => (0.0, 6.0 * progress, 1.0),
        6 => (0.0, 0.0, 1.0 - 0.02 * progress),
        _ => (0.0, 0.0, 1.0),
    };
    let floating = float(content.compose())
        .scale(scale)
        .translate(move |_, _| iced::Vector::new(x, y))
        .boxed();
    let shade = container(space::horizontal())
        .width(Fill)
        .height(Fill)
        .style(move |theme| Style {
            background: Some(theme.neutral.base.alpha(progress).into()),
            ..Style::default()
        });
    stack([floating, shade.into()]).boxed().into()
}
type ButtonStyle = StyleFn<'static, Theme>;

pub struct MessageScrollerPage {
    state: page::State,
}

impl MessageScrollerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for MessageScrollerPage {
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
        let mode = match id {
            "demo" => Demo::Chat,
            "anchoring" => Demo::Anchoring,
            "group-chat" => Demo::Group,
            "previous-context" => Demo::Context,
            "streaming" => Demo::Streaming,
            "opening-position" => Demo::Opening,
            "load-history" => Demo::History,
            "animation" => Demo::Animation,
            "commands" => Demo::Commands,
            "visibility" => Demo::Visibility,
            "scrollable" => Demo::Scrollable,
            _ => unreachable!("unknown message-scroller preview id: {id}"),
        };
        let index = self.page_index();
        component(mode)
            .boxed()
            .map(move |event| Message::Page(index, event))
            .boxed()
    }
}
