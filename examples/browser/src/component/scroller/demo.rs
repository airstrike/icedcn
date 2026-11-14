use super::fixtures::{self, Role, Turn};
use crate::page;
use iced::time::{Duration, Instant};
use iced::widget::component::Component;
use iced::widget::{Action, Id};
use iced::{Rectangle, Widget, mouse, window};
use icedcn::Theme;
use icedcn::widget::motion;
use icedcn::widget::scroller::{Animation, Position, Reader, Target};

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
                "See which directions you can scroll from your current position.",
            ),
        }
    }
    pub(super) fn caption(self) -> &'static str {
        match self {
            Self::Chat => "Demo is read only. Press send to send messages.",
            Self::Anchoring => {
                "Toggle the anchor role, then send messages to compare where turns settle."
            }
            Self::Group => "Add Rocky to start a turn anchored at the join marker.",
            Self::Context => {
                "Adjust the slider, then send a message to see how much of the previous reply stays visible."
            }
            Self::Streaming => {
                "The simulated reply follows the bottom of the viewport while you read there."
            }
            Self::Opening => {
                "Choose where the transcript opens. Try its start, its end, or its last anchored turn."
            }
            Self::History => "Click Load History to load the entire conversation",
            Self::Animation => "Select an animation then click send to see it in action.",
            Self::Commands => "Use the controls to jump to any message in the conversation.",
            Self::Visibility => "Open the outline to jump between anchored turns as you read.",
            Self::Scrollable => "Scroll the transcript to see the footer update.",
        }
    }
    pub(super) fn initial_first(self) -> usize {
        if self == Self::History {
            self.turns().len().saturating_sub(5)
        } else {
            0
        }
    }

    pub(super) fn initial_count(self) -> usize {
        match self {
            Self::Chat | Self::Anchoring | Self::Streaming | Self::Animation => 0,
            Self::Group => 3,
            Self::Context => 2,
            _ => self.turns().len(),
        }
    }

    fn initial(self) -> Model {
        Model {
            initialized: true,
            id: Id::unique(),
            count: self.initial_count(),
            first: self.initial_first(),
            peek: 64,
            preset: if self == Self::Animation {
                motion::Preset::Fade
            } else {
                motion::Preset::SlideUp
            },
            opening: match self {
                Self::Opening => Position::LastAnchor,
                Self::Scrollable => Position::Start,
                _ => Position::End,
            },
            reader: Reader::default(),
            reply: None,
            assistant_anchor: false,
            newest: None,
        }
    }
    fn interval(self) -> Option<Duration> {
        let millis = match self {
            Self::Chat | Self::Streaming => 20,
            Self::Context => 35,
            Self::Animation => 15,
            _ => return None,
        };
        Some(Duration::from_millis(millis))
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
    Tick(Instant),
    Reader(Reader),
    Peek(f32),
    Anchor(String),
    Open(String),
    Preset(motion::Preset),
    Jump(Target),
}
pub(super) struct Model {
    initialized: bool,
    pub(super) first: usize,
    pub(super) count: usize,
    pub(super) peek: u16,
    pub(super) assistant_anchor: bool,
    pub(super) preset: motion::Preset,
    pub(super) opening: Position,
    pub(super) id: Id,
    pub(super) reader: Reader,
    reply: Option<Reply>,
    pub(super) newest: Option<usize>,
}
struct Reply {
    due: Instant,
    interval: Duration,
    // The transport waits before it even mounts the assistant row.
    written: Option<usize>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            initialized: false,
            ..Demo::Chat.initial()
        }
    }
}

impl Model {
    pub(super) fn busy(&self) -> bool {
        self.reply.is_some()
    }

    pub(super) fn next(&self, demo: Demo) -> Option<&'static Turn> {
        let remaining = &demo.turns()[self.count..];
        if demo.interval().is_some() {
            remaining.iter().find(|turn| turn.role == Role::User)
        } else {
            remaining.first()
        }
    }

    pub(super) fn text(&self, demo: Demo, index: usize) -> &'static str {
        let text = demo.turns()[index].text;
        if index + 1 == self.count
            && let Some(written) = self.reply.as_ref().and_then(|r| r.written)
        {
            &text[..written]
        } else {
            text
        }
    }

    fn send(&mut self, demo: Demo, now: Instant) {
        if self.busy() || self.count == demo.turns().len() {
            return;
        }
        self.newest = Some(self.count);
        self.count += 1;
        if let Some(interval) = demo.interval() {
            self.reply = Some(Reply {
                due: now + Duration::from_secs(1),
                interval,
                written: None,
            });
        }
    }

    fn advance(&mut self, demo: Demo, now: Instant) {
        let Some(reply) = &mut self.reply else { return };
        while now >= reply.due {
            if let Some(written) = &mut reply.written {
                let text = demo.turns()[self.count - 1].text;
                // The transport emits /\S+\s*/: a word and its trailing whitespace.
                let rest = &text[*written..];
                let whitespace = rest.find(char::is_whitespace).unwrap_or(rest.len());
                let remaining = rest[whitespace..].trim_start();
                *written = text.len() - remaining.len();
                if remaining.is_empty() {
                    self.reply = None;
                    return;
                }
            } else {
                self.count += 1;
                reply.written = Some(0);
            }
            reply.due += reply.interval;
        }
    }

    fn open_at(&mut self, demo: Demo, position: Position) -> Target {
        self.opening = position;
        match position {
            Position::Start => Target::Start,
            Position::End => Target::End,
            Position::LastAnchor => (self.first..self.count)
                .rfind(|&i| demo.anchor(self, i))
                .map_or(Target::End, |i| Target::Item(i as u64)),
        }
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
        match event {
            Event::Reset if !state.busy() => {
                let mut reset = self.initial();
                reset.assistant_anchor = state.assistant_anchor;
                reset.preset = state.preset;
                reset.id = state.id.clone();
                *state = reset;
                if matches!(self, Self::Context | Self::Group | Self::History) {
                    return Some(page::Message::ScrollTranscript(
                        state.id.clone(),
                        Target::End,
                        Animation::Instant,
                    ));
                }
            }
            Event::Reset => {}
            Event::Send if *self == Demo::History => {
                state.first = 0;
                return Some(page::Message::Notice(
                    "History loaded".into(),
                    "Scroll up to see earlier messages.".into(),
                ));
            }
            Event::Send => state.send(*self, Instant::now()),
            Event::Tick(now) => state.advance(*self, now),
            Event::Reader(reader) => state.reader = reader,
            Event::Peek(px) => state.peek = px as u16,
            Event::Anchor(role) => {
                let assistant = role == "Assistant";
                if state.assistant_anchor != assistant {
                    *state = self.initial();
                    state.assistant_anchor = assistant;
                }
            }
            Event::Preset(preset) => state.preset = preset,
            Event::Open(pos) => {
                let position = match pos.as_str() {
                    "start" => Position::Start,
                    "end" => Position::End,
                    _ => Position::LastAnchor,
                };
                let target = state.open_at(*self, position);
                return Some(page::Message::ScrollTranscript(
                    state.id.clone(),
                    target,
                    Animation::Instant,
                ));
            }
            Event::Jump(target) => {
                return Some(page::Message::ScrollTranscript(
                    state.id.clone(),
                    target,
                    Animation::Smooth,
                ));
            }
        }
        None
    }
    fn listen(
        &self,
        state: &Model,
        event: &iced::Event,
        _: Rectangle,
        _: mouse::Cursor,
    ) -> Action<Event> {
        let (Some(reply), iced::Event::Window(window::Event::RedrawRequested(now))) =
            (&state.reply, event)
        else {
            return Action::none();
        };
        if *now >= reply.due {
            Action::publish(Event::Tick(*now)).and_request_redraw_at(reply.due + reply.interval)
        } else {
            Action::request_redraw_at(reply.due)
        }
    }

    fn view(&self, state: &Model) -> impl Widget<Event, Theme> + 'a {
        self.frame(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_scripted_send_waits_one_second_then_streams_words_at_its_own_rate() {
        for (demo, millis) in [
            (Demo::Chat, 20),
            (Demo::Context, 35),
            (Demo::Streaming, 20),
            (Demo::Animation, 15),
        ] {
            let mut model = demo.initial();
            let mut now = Instant::now();
            while model.next(demo).is_some() {
                let prompt = model.count;
                let turns = demo.turns();
                model.send(demo, now);
                model.send(demo, now);
                model.advance(demo, now + Duration::from_millis(999));
                assert_eq!(model.count, prompt + 1);
                assert_eq!(model.text(demo, prompt), turns[prompt].text);
                assert_eq!(
                    model.next(demo).map(|t| t.text),
                    turns.get(prompt + 2).map(|t| t.text)
                );
                assert!(model.busy());

                now += Duration::from_secs(1);
                model.advance(demo, now);
                assert_eq!(model.count, prompt + 2);
                assert_eq!(model.text(demo, prompt + 1), "");
                let full = turns[prompt + 1].text;
                let words: Vec<_> = full.split_whitespace().collect();
                let mut leading = 0;
                for (i, word) in words.iter().enumerate() {
                    leading += word.len();
                    leading = full.len() - full[leading..].trim_start().len();
                    now += Duration::from_millis(millis);
                    model.advance(demo, now);
                    assert_eq!(model.text(demo, prompt + 1), &full[..leading]);
                    assert_eq!(model.busy(), i + 1 < words.len());
                }
            }
            assert_eq!(model.count, demo.turns().len());
        }
    }

    #[test]
    fn missed_redraws_catch_up_without_changing_transport_speed() {
        let demo = Demo::Chat;
        let mut model = demo.initial();
        let now = Instant::now();
        model.send(demo, now);
        model.advance(demo, now + Duration::from_millis(1050));
        assert_eq!(model.text(demo, 1), "That's the ");
        model.advance(demo, now + Duration::from_secs(15));
        assert!(!model.busy());
        assert_eq!(model.text(demo, 1), demo.turns()[1].text);
    }

    #[test]
    fn manual_examples_append_one_complete_row_per_click() {
        for demo in [Demo::Anchoring, Demo::Group] {
            let mut model = demo.initial();
            let rows = demo.turns();
            while model.count < rows.len() {
                let i = model.count;
                model.send(demo, Instant::now());
                assert_eq!(model.count, i + 1);
                assert_eq!(model.text(demo, i), rows[i].text);
                assert!(!model.busy());
            }
        }
    }

    #[test]
    fn saved_examples_open_at_their_recipe_position() {
        for (demo, count, position) in [
            (Demo::Context, 2, Position::End),
            (Demo::Group, 3, Position::End),
            (Demo::Commands, 8, Position::End),
            (Demo::Visibility, 8, Position::End),
            (Demo::Scrollable, 12, Position::Start),
            (Demo::History, 10, Position::End),
            (Demo::Opening, 4, Position::LastAnchor),
        ] {
            let model = demo.initial();
            assert_eq!(model.count, count);
            assert_eq!(model.opening, position);
            if demo == Demo::History {
                assert_eq!(model.first, 5);
            } else if demo == Demo::Opening {
                let mut model = model;
                assert_eq!(model.open_at(demo, Position::LastAnchor), Target::Item(2));
            }
        }
    }
}
