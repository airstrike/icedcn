use super::fixtures::{self, Role, Turn};
use crate::page;
use iced::Widget;
use iced::widget::component::Component;
use icedxdy::Theme;
use icedxdy::widget::motion;
use icedxdy::widget::scroller::{Command, Position, Reader, Target};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Demo {
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
    pub(super) fn turns(self) -> &'static [Turn] {
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
    pub(super) fn info(self) -> (&'static str, &'static str) {
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
    pub(super) fn caption(self) -> &'static str {
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
    pub(super) fn initial(self) -> Model {
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
    pub(super) fn anchor(self, state: &Model, i: usize) -> bool {
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
pub(super) enum Event {
    Send,
    Reset,
    Tick,
    Reader(Reader),
    Peek(f32),
    Anchor(String),
    Open(String),
    Preset(motion::Preset),
    Jump(Target),
}
#[derive(Default)]
pub(super) struct Model {
    initialized: bool,
    pub(super) first: usize,
    pub(super) count: usize,
    pub(super) peek: u16,
    pub(super) assistant_anchor: bool,
    pub(super) preset: motion::Preset,
    pub(super) opening: Position,
    pub(super) command: Option<Command>,
    pub(super) reader: Reader,
    pub(super) stream: Option<usize>,
    pub(super) newest: Option<usize>,
}
impl Model {
    fn jump(&mut self, target: Target) {
        let sequence = self.command.map_or(1, |cmd| cmd.sequence.wrapping_add(1));
        self.command = Some(Command { sequence, target });
    }
}
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
                state.newest = Some(index);
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
            }
            Event::Reader(reader) => state.reader = reader,
            Event::Peek(px) => state.peek = px as u16,
            Event::Anchor(role) => {
                *state = self.initial();
                state.assistant_anchor = role == "Assistant";
            }
            Event::Preset(preset) => state.preset = preset,
            Event::Open(pos) => {
                state.opening = match pos.as_str() {
                    "start" => Position::Start,
                    "end" => Position::End,
                    _ => Position::LastAnchor,
                };
                state.jump(match state.opening {
                    Position::Start => Target::Start,
                    Position::End => Target::End,
                    Position::LastAnchor => (state.first..state.count)
                        .rfind(|&i| self.anchor(state, i))
                        .map_or(Target::Start, |i| Target::Item(i as u64)),
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
