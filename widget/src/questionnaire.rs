//! A multi-step native form with optional, conditional and multiple-choice questions.
mod body;
mod model;

pub use model::{Answer, Answers, Choice, Item, choice, item};

use crate::column::Column;
use crate::compose::{Compose, Inherited, View};
use crate::tabs::list::List;
use crate::{button, checkbox, column, container, field, input, kbd, radio, row, space, text};
use iced_core::Length::Fill;
use iced_core::{Element, window};
use iced_widget::{
    Renderer, Widget,
    component::{self, Component},
};
use icedcn_core::{Theme, theme::text_input};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Shortcuts {
    #[default]
    None,
    Letters,
    Numbers,
}

impl Shortcuts {
    fn key(self, i: usize) -> Option<char> {
        match self {
            Self::Letters if i < 26 => Some((b'A' + i as u8) as char),
            Self::Numbers if i < 9 => Some((b'1' + i as u8) as char),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Invalid {
    pub item: String,
    pub message: String,
}

/// Item position is one-based and excludes disabled items.
#[derive(Clone, Copy, Debug)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
}

pub struct Questionnaire<'a, M> {
    items: Vec<Item>,
    initial_answers: Answers,
    default_item: Option<String>,
    active: Option<String>,
    initial_reset: Option<String>,
    on_reset: Option<Box<dyn Fn() -> M + 'a>>,
    shortcuts: Shortcuts,
    skip: bool,
    block_unanswered: bool,
    submit: String,
    disabled: bool,
    animate: bool,
    progress: Option<Box<dyn Fn(Progress) -> View<'a, Event> + 'a>>,
    on_item: Option<Box<dyn Fn(String) -> M + 'a>>,
    on_submit: Option<Box<dyn Fn(Answers) -> M + 'a>>,
    validate: Option<Validation<'a>>,
    controlled_answers: Option<Answers>,
    on_answers: Option<Box<dyn Fn(Answers) -> M + 'a>>,
}

/// Build a form from its questions in display order. Empty forms render no controls.
pub fn questionnaire<'a, M>(items: impl IntoIterator<Item = Item>) -> Questionnaire<'a, M> {
    Questionnaire {
        items: items.into_iter().collect(),
        initial_answers: Answers::new(),
        default_item: None,
        active: None,
        initial_reset: None,
        on_reset: None,
        shortcuts: Shortcuts::None,
        skip: false,
        block_unanswered: false,
        submit: "Submit".into(),
        disabled: false,
        animate: false,
        progress: Some(Box::new(|p| {
            text::muted(format!("Question {} of {}", p.current, p.total))
                .size(12)
                .into()
        })),
        on_item: None,
        on_submit: None,
        validate: None,
        controlled_answers: None,
        on_answers: None,
    }
}

impl<'a, M> Questionnaire<'a, M> {
    /// Seed an uncontrolled form, including responses from a saved draft.
    pub fn default_answers(mut self, answers: Answers) -> Self {
        self.initial_answers = answers;
        self
    }
    /// Supply host-owned answers. Each edit publishes the entire updated answer map.
    pub fn answers(mut self, answers: Answers) -> Self {
        self.controlled_answers = Some(answers);
        self
    }
    pub fn on_answers_change(mut self, change: impl Fn(Answers) -> M + 'a) -> Self {
        self.on_answers = Some(Box::new(change));
        self
    }
    pub fn default_item(mut self, item: impl Into<String>) -> Self {
        self.default_item = Some(item.into());
        self
    }
    /// Specify the active item by name. Navigation publishes its name to the host.
    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.active = Some(item.into());
        self
    }
    pub fn on_item_change(mut self, f: impl Fn(String) -> M + 'a) -> Self {
        self.on_item = Some(Box::new(f));
        self
    }
    pub fn shortcuts(mut self, shortcuts: Shortcuts) -> Self {
        self.shortcuts = shortcuts;
        self
    }
    pub fn allow_skip(mut self, allow: bool) -> Self {
        self.skip = allow;
        self
    }
    pub fn reset_action(mut self, label: impl Into<String>) -> Self {
        self.initial_reset = Some(label.into());
        self
    }
    pub fn on_reset(mut self, f: impl Fn() -> M + 'a) -> Self {
        self.on_reset = Some(Box::new(f));
        self
    }
    pub fn block_unanswered(mut self, disabled: bool) -> Self {
        self.block_unanswered = disabled;
        self
    }
    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit = label.into();
        self
    }
    /// Validate all enabled answers on submit. An error navigates to its owning item.
    pub fn validate(mut self, f: impl Fn(&Answers) -> Option<Invalid> + 'a) -> Self {
        self.validate = Some(Box::new(f));
        self
    }
    pub fn on_submit(mut self, f: impl Fn(Answers) -> M + 'a) -> Self {
        self.on_submit = Some(Box::new(f));
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn animated(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
    /// Render the current position among enabled items.
    pub fn progress(mut self, f: impl Fn(Progress) -> View<'a, Event> + 'a) -> Self {
        self.progress = Some(Box::new(f));
        self
    }
    pub fn hide_progress(mut self) -> Self {
        self.progress = None;
        self
    }
}

#[derive(Default)]
pub struct State {
    active: Option<String>,
    answers: Option<Answers>,
    error: Option<Invalid>,
    pending_item: Option<String>,
    pending_reset: bool,
}

#[derive(Clone, Debug)]
pub enum Event {
    Choose(usize),
    Write(String),
    Previous,
    Next,
    Skip,
    Reset,
    Submit,
    NotifyItem,
    NotifyReset,
}

impl<'a, M> Questionnaire<'a, M> {
    fn values<'s>(&'s self, state: &'s State) -> &'s Answers {
        self.controlled_answers
            .as_ref()
            .or(state.answers.as_ref())
            .unwrap_or(&self.initial_answers)
    }
    fn active_index(&self, state: &State, enabled: &[&Item]) -> Option<usize> {
        let desired = self
            .active
            .as_ref()
            .or(state.active.as_ref())
            .or(self.default_item.as_ref());
        desired
            .and_then(|v| enabled.iter().position(|i| &i.name == v))
            .or_else(|| (!enabled.is_empty()).then_some(0))
    }
    fn go(&self, state: &mut State, name: String) -> Option<M> {
        if self.active.is_none() {
            state.active = Some(name.clone());
        }
        self.on_item.as_ref().map(|f| f(name))
    }
    fn save(&self, state: &mut State, answers: Answers) -> Option<M> {
        if self.controlled_answers.is_none() {
            state.answers = Some(answers.clone());
        }
        self.on_answers.as_ref().map(|f| f(answers))
    }
    // A reset or skip may change both controlled props. A Component forwards one
    // parent message per event, so forward the item change on the next redraw.
    fn save_and_go(&self, state: &mut State, answers: Answers, item: String) -> Option<M> {
        let change = self.save(state, answers);
        if change.is_some() && self.on_item.is_some() {
            if self.active.is_none() {
                state.active = Some(item.clone());
            }
            state.pending_item = Some(item);
            change
        } else {
            self.go(state, item).or(change)
        }
    }
    fn invalid(&self, state: &mut State, issue: Invalid) -> Option<M> {
        let name = issue.item.clone();
        state.error = Some(issue);
        self.go(state, name)
    }
}
impl<M> Questionnaire<'_, M> {
    fn apply(&self, state: &mut State, event: Event) -> Option<M> {
        if let Event::NotifyReset = event {
            state.pending_reset = false;
            return self.on_reset.as_ref().map(|f| f());
        }
        if let Event::NotifyItem = event {
            return self
                .on_item
                .as_ref()
                .and_then(|f| state.pending_item.take().map(f));
        }
        if self.disabled {
            return None;
        }
        let mut answers = self.values(state).clone();
        let enabled: Vec<_> = self.items.iter().filter(|i| i.enabled(&answers)).collect();
        let pos = self.active_index(state, &enabled)?;
        let current = enabled[pos];
        state.error = None;
        match event {
            Event::Choose(i) => {
                current.choose(answers.entry(current.name.clone()).or_default(), i);
                self.save(state, answers)
            }
            Event::Write(s) => {
                current.write(answers.entry(current.name.clone()).or_default(), s);
                self.save(state, answers)
            }
            Event::Reset => {
                let active = self
                    .default_item
                    .clone()
                    .or_else(|| enabled.first().map(|i| i.name.clone()))?;
                let changed = self.save_and_go(state, self.initial_answers.clone(), active);
                if changed.is_some() {
                    state.pending_reset = self.on_reset.is_some();
                    changed
                } else {
                    self.on_reset.as_ref().map(|f| f())
                }
            }
            Event::Previous => self.go(state, enabled.get(pos.checked_sub(1)?)?.name.clone()),
            Event::Skip if self.skip && !current.required => {
                answers.insert(
                    current.name.clone(),
                    Answer {
                        skipped: true,
                        ..Answer::default()
                    },
                );
                if let Some(next) = enabled.get(pos + 1) {
                    self.save_and_go(state, answers, next.name.clone())
                } else {
                    self.save(state, answers)
                }
            }
            Event::Next | Event::Submit => {
                let submit = matches!(event, Event::Submit);
                let invalid = enabled.iter().copied().find(|i| {
                    (submit || i.name == current.name)
                        && i.required
                        && !answers.get(&i.name).is_some_and(Answer::answered)
                });
                if let Some(invalid) = invalid {
                    return self.invalid(
                        state,
                        Invalid {
                            item: invalid.name.clone(),
                            message: "Please answer this question.".into(),
                        },
                    );
                }
                if !submit {
                    return self.go(state, enabled.get(pos + 1)?.name.clone());
                }
                answers.retain(|name, _| enabled.iter().any(|i| &i.name == name));
                if let Some(issue) = self.validate.as_ref().and_then(|f| f(&answers)) {
                    return self.invalid(state, issue);
                }
                self.on_submit.as_ref().map(|f| f(answers))
            }
            Event::Skip | Event::NotifyItem | Event::NotifyReset => None,
        }
    }
}
impl<'a, M: 'static> Component<'a, M, Theme, Renderer> for Questionnaire<'a, M> {
    type State = State;
    type Event = Event;
    fn listen(
        &self,
        state: &State,
        event: &iced_core::Event,
        _: iced_core::Rectangle,
        _: iced_core::mouse::Cursor,
    ) -> iced_widget::Action<Event> {
        if matches!(
            event,
            iced_core::Event::Window(window::Event::RedrawRequested(_))
        ) {
            if state.pending_item.is_some() {
                return iced_widget::Action::publish(Event::NotifyItem);
            }
            if state.pending_reset {
                return iced_widget::Action::publish(Event::NotifyReset);
            }
        }
        iced_widget::Action::none()
    }
    fn update(&self, state: &mut State, event: Event, _: &Renderer) -> Option<M> {
        self.apply(state, event)
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let answers = self.values(state);
        let enabled: Vec<_> = self.items.iter().filter(|i| i.enabled(answers)).collect();
        let Some(pos) = self.active_index(state, &enabled) else {
            return Element::from(column![]);
        };
        let item = enabled[pos];
        let empty = Answer::default();
        let value = answers.get(&item.name).unwrap_or(&empty);
        let mut blocks = Vec::new();
        if let Some(render) = &self.progress {
            blocks.push(render(Progress {
                current: pos + 1,
                total: enabled.len(),
            }));
        }
        let mut parts: Vec<View<'a, Event>> = vec![text::label(item.title.clone()).size(16).into()];
        if let Some(desc) = &item.description {
            parts.push(text::muted(desc.clone()).into());
        }
        let mut choices = Vec::new();
        for (i, choice) in item.choices.iter().enumerate() {
            let selected = value.contains(&choice.value);
            let control: View<'a, Event> = if item.multiple {
                checkbox()
                    .checked(selected)
                    .on_checked_change(move |_| Event::Choose(i))
                    .into()
            } else {
                radio("", i, selected.then_some(i), Event::Choose)
                    .size(16.)
                    .into()
            };
            let mut labels = column![text::label(choice.label.clone())]
                .width(Fill)
                .spacing(2);
            if let Some(detail) = &choice.description {
                labels = labels.push(text::muted(detail.clone()).into());
            }
            let mut line = row![labels].width(Fill).spacing(8);
            if let Some(key) = self.shortcuts.key(i) {
                line = line.push(kbd(key.to_string()).into());
            }
            choices.push(
                field(control)
                    .label(View::from(line))
                    .orientation(field::Orientation::Horizontal)
                    .card()
                    .width(Fill)
                    .into(),
            );
        }
        let choices = Column::with_children(choices).width(Fill).spacing(8);
        let fixed: View<'a, Event> = if item.multiple {
            choices.into()
        } else {
            List {
                content: choices.into(),
                orientation: None,
                single_tab_stop: false,
            }
            .boxed()
            .into()
        };
        let mut inputs = column![fixed].spacing(8).width(Fill);
        if let Some((name, placeholder)) = &item.input {
            let mut other = input(if placeholder.is_empty() {
                name.clone()
            } else {
                placeholder.clone()
            })
            .width(Fill)
            .value(value.text.clone())
            .on_input(Event::Write);
            if state
                .error
                .as_ref()
                .is_some_and(|issue| issue.item == item.name)
            {
                other = other.style(text_input::Invalid);
            }
            inputs = inputs.push(other.into());
        }
        parts.push(inputs.into());
        if let Some(issue) = &state.error
            && issue.item == item.name
        {
            parts.push(text::error(issue.message.clone()).into());
        }
        let inner = Column::with_children(parts).spacing(16).width(Fill);
        let node = body::Body {
            content: inner.into(),
            shortcuts: self.shortcuts,
            choices: item.choices.len(),
            step: item.name.clone(),
            animate: self.animate,
            invalid: state
                .error
                .as_ref()
                .is_some_and(|issue| issue.item == item.name),
        };
        blocks.push(node.boxed().into());
        let mut actions = row![].spacing(8).width(Fill);
        if let Some(label) = &self.initial_reset {
            actions = actions.push(
                button(label.clone())
                    .style(button::Outline)
                    .on_press(Event::Reset)
                    .into(),
            );
        }
        if pos > 0 {
            actions = actions.push(
                button("Previous")
                    .style(button::Outline)
                    .on_press(Event::Previous)
                    .into(),
            );
        }
        actions = actions.push(space::horizontal().into());
        if self.skip && !item.required && pos + 1 < enabled.len() {
            actions = actions.push(
                button("Skip")
                    .style(button::Outline)
                    .on_press(Event::Skip)
                    .into(),
            );
        }
        let blocked = self.block_unanswered && !value.answered();
        if pos + 1 < enabled.len() {
            let mut next = button("Next").on_press(Event::Next).disabled(blocked);
            if self.block_unanswered {
                next = next.style(button::Secondary);
            }
            actions = actions.push(next.into());
        } else {
            actions = actions.push(
                button(self.submit.clone())
                    .on_press(Event::Submit)
                    .disabled(blocked)
                    .into(),
            );
        }
        blocks.push(actions.into());
        let mut root = View::from(
            container(Column::with_children(blocks).spacing(16).width(Fill)).width(Fill),
        );
        root.inherit(&Inherited::default().with_disabled(self.disabled));
        root.compose()
    }
}

impl<'a, M: 'static> Compose<M> for Questionnaire<'a, M> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, M, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, M: 'static> From<Questionnaire<'a, M>> for View<'a, M> {
    fn from(q: Questionnaire<'a, M>) -> Self {
        Self::new(q)
    }
}
impl<'a, M: 'static> From<Questionnaire<'a, M>> for Element<'a, M, Theme, Renderer> {
    fn from(q: Questionnaire<'a, M>) -> Self {
        Box::new(q).compose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Change {
        Answers(Answers),
        Item(String),
        Reset,
    }

    #[test]
    fn controlled_skip_and_reset_deliver_every_change_and_keep_the_host_in_control() {
        let make = || {
            questionnaire([item("one", "One"), item("two", "Two")])
                .item("one")
                .default_item("one")
                .answers(Answers::new())
                .allow_skip(true)
                .on_item_change(Change::Item)
                .on_answers_change(Change::Answers)
                .on_reset(|| Change::Reset)
        };
        let form = make();
        let mut state = State::default();
        let Some(Change::Answers(answers)) = form.apply(&mut state, Event::Skip) else {
            panic!("missing answers")
        };
        assert!(answers["one"].skipped);
        assert!(state.answers.is_none() && state.active.is_none());
        assert_eq!(
            form.apply(&mut state, Event::NotifyItem),
            Some(Change::Item("two".into()))
        );
        assert_eq!(
            form.apply(&mut state, Event::Reset),
            Some(Change::Answers(Answers::new()))
        );
        assert_eq!(
            form.apply(&mut state, Event::NotifyItem),
            Some(Change::Item("one".into()))
        );
        assert_eq!(
            form.apply(&mut state, Event::NotifyReset),
            Some(Change::Reset)
        );
    }
}

type Validation<'a> = Box<dyn Fn(&Answers) -> Option<Invalid> + 'a>;
