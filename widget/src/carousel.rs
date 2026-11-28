//! A snapping carousel with persistent slides, pointer dragging and keyboard controls.
mod icon;
mod viewport;

use std::time::Duration;

use iced_core::{Alignment, Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedcn_core::{Orientation, Theme};

use crate::compose::{Compose, Inherited, View};
use crate::{Size, button, column, text};

/// A slide factory, navigation controls and an optional controlled snap index.
/// All slides stay mounted; selection translates a clipped track.
pub struct Carousel<'a, Message> {
    count: usize,
    slide: Box<dyn Fn(usize) -> View<'a, Message> + 'a>,
    visible: usize,
    axis: Orientation,
    align: Alignment,
    looping: bool,
    width: Length,
    height: Option<f32>,
    default: usize,
    value: Option<usize>,
    change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    gap: f32,
    autoplay: Option<Duration>,
    stop_on_hover: bool,
    inherited: Inherited,
    caption: bool,
    prev: Option<Box<dyn Fn() -> View<'a, Message> + 'a>>,
    next: Option<Box<dyn Fn() -> View<'a, Message> + 'a>>,
}

/// Build `count` slides. The factory receives a stable zero-based slide index.
pub fn carousel<'a, Message: 'a, S>(
    count: usize,
    slide: impl Fn(usize) -> S + 'a,
) -> Carousel<'a, Message>
where
    S: Into<View<'a, Message>> + 'a,
{
    Carousel {
        count,
        slide: Box::new(move |i| slide(i).into()),
        visible: 1,
        axis: Orientation::Horizontal,
        align: Alignment::Center,
        looping: false,
        width: Length::Fill,
        height: None,
        default: 0,
        value: None,
        change: None,
        gap: 16.0,
        autoplay: None,
        stop_on_hover: false,
        inherited: Inherited::default(),
        caption: false,
        prev: None,
        next: None,
    }
}

impl<'a, Message: 'a> Carousel<'a, Message> {
    /// Track direction; defaults to horizontal.
    pub fn orientation(mut self, axis: Orientation) -> Self {
        self.axis = axis;
        self
    }

    /// Number of equal-width (or equal-height) slides in the viewport; defaults to one.
    pub fn visible(mut self, count: usize) -> Self {
        self.visible = count.max(1);
        self
    }

    /// Alignment at a snap. Non-looping tracks contain their first and last edges.
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    /// Wrap navigation and dragging. Falls back to a contained track if all slides fit.
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Track width, excluding the 48px horizontal control gutters on either side.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Track height, excluding vertical control gutters. Otherwise uses slide height.
    pub fn height(mut self, pixels: f32) -> Self {
        self.height = Some(pixels.max(0.0));
        self
    }

    /// Gap between slide cells; defaults to 16 logical pixels.
    pub fn spacing(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Seed the uncontrolled zero-based snap index once.
    pub fn default_value(mut self, index: usize) -> Self {
        self.default = index;
        self
    }

    /// Display an application-owned snap. Interaction only publishes a requested index.
    pub fn value(mut self, index: usize) -> Self {
        self.value = Some(index);
        self
    }

    /// Publish the complete requested snap index after selection changes.
    pub fn on_value_change(mut self, change: impl Fn(usize) -> Message + 'a) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    /// Disable the carousel and its descendants independently of any callbacks.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inherited = self.inherited.with_disabled(disabled);
        self
    }

    /// Advance on scheduled redraws. Hover pauses; dragging or focusing a slide stops it.
    /// Keyboard navigation buttons do not stop the timer. Zero disables autoplay.
    pub fn autoplay(mut self, delay: Duration) -> Self {
        self.autoplay = (!delay.is_zero()).then_some(delay);
        self
    }

    /// Stop autoplay permanently when the pointer enters the carousel.
    /// By default, leaving the carousel starts a fresh delay after a hover pause.
    pub fn stop_on_hover(mut self, stop: bool) -> Self {
        self.stop_on_hover = stop;
        self
    }

    /// Display a built-in snap counter. Use `on_value_change` for application-owned copy.
    pub fn caption(mut self, visible: bool) -> Self {
        self.caption = visible;
        self
    }

    /// Override the previous and next buttons' content. Default icons are chevrons.
    pub fn controls<S: Into<View<'a, Message>> + 'a>(
        mut self,
        prev: impl Fn() -> S + 'a,
        next: impl Fn() -> S + 'a,
    ) -> Self {
        self.prev = Some(Box::new(move || prev().into()));
        self.next = Some(Box::new(move || next().into()));
        self
    }

    /// Number of selectable snaps, including contained endpoints.
    pub fn snap_count(&self) -> usize {
        self.snaps().len()
    }

    fn loops(&self) -> bool {
        self.looping && self.count > self.visible
    }

    fn snaps(&self) -> Vec<f32> {
        snaps(self.count, self.visible, self.align, self.loops())
    }

    fn current(&self, state: &State) -> usize {
        self.value
            .or(state.index)
            .unwrap_or(self.default)
            .min(self.snap_count().saturating_sub(1))
    }
}

/// Internal selection state retained by iced's Component tree.
#[derive(Default)]
pub struct State {
    index: Option<usize>,
    direction: i8,
}

/// Internal events; application messages cross the Component boundary unchanged.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    Previous,
    Next,
    Select(usize),
    Advance,
    App(Message),
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme> for Carousel<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        let last = self.snap_count().saturating_sub(1);
        let index = state.index.get_or_insert(self.default);
        *index = (*index).min(last);
    }

    fn update(&self, state: &mut State, event: Event<Message>, _: &Renderer) -> Option<Message> {
        if let Event::App(message) = event {
            return Some(message);
        }
        if self.inherited.is_disabled() {
            return None;
        }
        let current = self.current(state);
        let last = self.snap_count().saturating_sub(1);
        let (next, direction) = match event {
            Event::Previous if current == 0 && self.loops() => (last, -1),
            Event::Previous => (current.saturating_sub(1), -1),
            Event::Next if current == last && self.loops() => (0, 1),
            Event::Next => ((current + 1).min(last), 1),
            Event::Select(index) => (index.min(last), 0),
            Event::Advance if last > 0 => ((current + 1) % (last + 1), 1),
            Event::Advance | Event::App(_) => return None,
        };
        if current == next {
            return None;
        }
        if self.value.is_none() {
            state.index = Some(next);
        }
        state.direction = direction;
        self.change.as_ref().map(|f| f(next))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme> + 'a {
        let index = self.current(state);
        let snaps = self.snaps();
        let last = snaps.len().saturating_sub(1);
        let disabled = self.inherited.is_disabled();
        let mut children = Vec::with_capacity(self.count + 2);
        for i in 0..self.count {
            let mut slide = (self.slide)(i);
            slide.inherit(&self.inherited);
            children.push(slide.map(Event::App).compose());
        }
        let arrow = |custom: &Option<Box<dyn Fn() -> View<'a, Message> + 'a>>,
                     next: bool,
                     blocked: bool,
                     event| {
            let mut content = custom
                .as_ref()
                .map(|f| f())
                .unwrap_or_else(|| icon::chevron(self.axis, next));
            content.inherit(&self.inherited);
            button(content.map(Event::App))
                .size(Size::Sm)
                .style(button::Outline)
                .rounded()
                .square()
                .width(28)
                .height(28)
                .padding(0)
                .disabled(disabled || blocked)
                .on_press(event)
                .into()
        };
        children.push(arrow(
            &self.prev,
            false,
            !self.loops() && index == 0,
            Event::Previous,
        ));
        children.push(arrow(
            &self.next,
            true,
            !self.loops() && index == last,
            Event::Next,
        ));
        let track = viewport::Viewport {
            children,
            count: self.count,
            visible: self.visible,
            width: self.width,
            height: self.height,
            axis: self.axis,
            index,
            snaps,
            looping: self.loops(),
            direction: state.direction,
            gap: self.gap,
            autoplay: self.autoplay.filter(|_| last > 0),
            stop_on_hover: self.stop_on_hover,
            disabled,
        }
        .boxed();
        if self.caption {
            column![
                track,
                text::muted(format!(
                    "Slide {} of {}",
                    if self.count == 0 { 0 } else { index + 1 },
                    self.snap_count()
                ))
                .size(14.0)
                .line_height(iced_core::Pixels(20.0))
                .height(20.0),
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .into()
        } else {
            track
        }
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Carousel<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: Clone + 'static> From<Carousel<'a, Message>> for View<'a, Message> {
    fn from(carousel: Carousel<'a, Message>) -> Self {
        View::new(carousel)
    }
}

impl<'a, Message: Clone + 'static> From<Carousel<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(carousel: Carousel<'a, Message>) -> Self {
        Box::new(carousel).compose()
    }
}

// Align equally sized slides and trim snap points to the scrollable range.
fn snaps(count: usize, visible: usize, align: Alignment, looping: bool) -> Vec<f32> {
    if count == 0 {
        return Vec::new();
    }
    let end = count.saturating_sub(visible) as f32;
    let inset = match align {
        Alignment::Start => 0.0,
        Alignment::Center => (visible - 1) as f32 / 2.0,
        Alignment::End => (visible - 1) as f32,
    };
    let mut snaps = Vec::new();
    for i in 0..count {
        let value = i as f32 - inset;
        let value = if looping {
            value
        } else {
            value.clamp(0.0, end)
        };
        if snaps.last().copied() != Some(value) {
            snaps.push(value);
        }
    }
    snaps
}
