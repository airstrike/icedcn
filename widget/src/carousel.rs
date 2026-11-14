//! A carousel exposes the selected slide (snap index) and composes whichever
//! slides fit the visible window. Navigation uses buttons, arrows and swipes.
mod viewport;
use crate::compose::{Compose, Inherited, Slot};
use crate::{Size, button, column, row, text};
use iced_core::Element;
use iced_core::Length::Fill;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedcn_core::{Orientation, Theme};
use std::time::Duration;

pub struct Carousel<'a, Message> {
    count: usize,
    slide: Box<dyn Fn(usize) -> Slot<'a, Message> + 'a>,
    visible: usize,
    axis: Orientation,
    default: usize,
    value: Option<usize>,
    change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    gap: f32,
    autoplay: Option<Duration>,
    inherited: Inherited,
    caption: bool,
    prev: Option<Box<dyn Fn() -> Slot<'a, Message> + 'a>>,
    next: Option<Box<dyn Fn() -> Slot<'a, Message> + 'a>>,
}
/// Supply a slide factory so a move only composes the visible slides.
pub fn carousel<'a, Message: 'a, S>(
    count: usize,
    slide: impl Fn(usize) -> S + 'a,
) -> Carousel<'a, Message>
where
    S: Into<Slot<'a, Message>> + 'a,
{
    Carousel {
        count,
        slide: Box::new(move |i| slide(i).into()),
        visible: 1,
        axis: Orientation::Horizontal,
        default: 0,
        value: None,
        change: None,
        gap: 16.0,
        autoplay: None,
        inherited: Inherited::default(),
        caption: false,
        prev: None,
        next: None,
    }
}
impl<'a, Message: 'a> Carousel<'a, Message> {
    pub fn orientation(mut self, axis: Orientation) -> Self {
        self.axis = axis;
        self
    }
    pub fn visible(mut self, count: usize) -> Self {
        self.visible = count.max(1);
        self
    }
    pub fn spacing(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }
    pub fn default_value(mut self, index: usize) -> Self {
        self.default = index;
        self
    }
    pub fn value(mut self, index: usize) -> Self {
        self.value = Some(index);
        self
    }
    pub fn on_value_change(mut self, change: impl Fn(usize) -> Message + 'a) -> Self {
        self.change = Some(Box::new(change));
        self
    }
    pub fn autoplay(mut self, delay: Duration) -> Self {
        self.autoplay = (!delay.is_zero()).then_some(delay);
        self
    }
    pub fn caption(mut self, visible: bool) -> Self {
        self.caption = visible;
        self
    }
    pub fn controls<S: Into<Slot<'a, Message>> + 'a>(
        mut self,
        prev: impl Fn() -> S + 'a,
        next: impl Fn() -> S + 'a,
    ) -> Self {
        self.prev = Some(Box::new(move || prev().into()));
        self.next = Some(Box::new(move || next().into()));
        self
    }
    fn last(&self) -> usize {
        self.count.saturating_sub(self.visible)
    }
    fn current(&self, state: &State) -> usize {
        self.value
            .or(state.index)
            .unwrap_or(self.default)
            .min(self.last())
    }
}
#[derive(Default)]
pub struct State {
    index: Option<usize>,
    interacted: bool,
}
#[derive(Debug, Clone)]
pub enum Event<Message> {
    Previous,
    Next,
    Advance,
    App(Message),
}
impl<'a, Message: Clone + 'static> Component<'a, Message, Theme> for Carousel<'a, Message> {
    type State = State;
    type Event = Event<Message>;
    fn update(&self, state: &mut State, event: Event<Message>, _: &Renderer) -> Option<Message> {
        let current = self.current(state);
        let last = self.last();
        let next = match event {
            Event::App(m) => return Some(m),
            Event::Previous => {
                state.interacted = true;
                current.saturating_sub(1)
            }
            Event::Next => {
                state.interacted = true;
                (current + 1).min(last)
            }
            Event::Advance if !state.interacted && last > 0 => (current + 1) % (last + 1),
            Event::Advance => return None,
        };
        if current == next {
            return None;
        }
        if self.value.is_none() {
            state.index = Some(next);
        }
        self.change.as_ref().map(|f| f(next))
    }
    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme> + 'a {
        let index = self.current(state);
        let vertical = self.axis == Orientation::Vertical;
        let disabled = self.inherited.is_disabled();
        let mut slides = Vec::new();
        for i in index..(index + self.visible).min(self.count) {
            let mut slide = (self.slide)(i);
            slide.inherit(&self.inherited);
            slides.push(slide.map(Event::App));
        }
        let panel: Slot<'a, Event<Message>> = if vertical {
            column(slides).spacing(self.gap).width(Fill).into()
        } else {
            crate::Row::with_children(slides)
                .spacing(self.gap)
                .width(Fill)
                .into()
        };
        let arrow = |custom: &Option<Box<dyn Fn() -> Slot<'a, Message> + 'a>>,
                     fallback: &'static str,
                     blocked,
                     event| {
            let content = custom
                .as_ref()
                .map(|f| f())
                .unwrap_or_else(|| text::p(fallback).into())
                .map(Event::App);
            button(content)
                .size(Size::Sm)
                .style(button::Outline)
                .rounded()
                .square()
                .disabled(disabled || blocked)
                .on_press(event)
        };
        let prev = arrow(
            &self.prev,
            if vertical { "↑" } else { "←" },
            index == 0,
            Event::Previous,
        );
        let next = arrow(
            &self.next,
            if vertical { "↓" } else { "→" },
            index == self.last(),
            Event::Next,
        );
        let group: Slot<'a, Event<Message>> = if vertical {
            column![prev, panel, next]
                .spacing(12)
                .align_x(iced_core::Alignment::Center)
                .width(Fill)
                .into()
        } else {
            row![prev, panel, next].spacing(12).width(Fill).into()
        };
        let content = if self.caption {
            let n = if self.count == 0 { 0 } else { index + 1 };
            column![
                group,
                text::muted(format!("Slide {n} of {}", self.count)).size(14.0)
            ]
            .spacing(8)
            .align_x(iced_core::Alignment::Center)
            .width(Fill)
            .into()
        } else {
            group
        };
        viewport::Viewport {
            child: content.compose(),
            axis: self.axis,
            autoplay: self
                .autoplay
                .filter(|_| !state.interacted && self.last() > 0 && !disabled),
            disabled,
        }
    }
}
impl<'a, Message: Clone + 'static> Compose<Message> for Carousel<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = from.clone();
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, Message: Clone + 'static> From<Carousel<'a, Message>> for Slot<'a, Message> {
    fn from(c: Carousel<'a, Message>) -> Self {
        Slot::new(c)
    }
}
impl<'a, Message: Clone + 'static> From<Carousel<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(c: Carousel<'a, Message>) -> Self {
        component::component(c).boxed()
    }
}
