//! Sliders let users set a value by moving an indicator.
//!
//! # Example
//! ```ignore
//! use icedcn::widget::slider;
//!
//! struct State {
//!    value: f32,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ValueChanged(f32),
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     slider(0.0..=100.0)
//!         .value(state.value)
//!         .on_value_change(Message::ValueChanged)
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::ValueChanged(value) => {
//!             state.value = value;
//!         }
//!     }
//! }
//! ```
use crate::core::animation::Easing;
use crate::core::border::{self, Border};
use crate::core::keyboard;
use crate::core::keyboard::key::{self, Key};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::time::Instant;
use crate::core::touch;
use crate::core::widget::Meta;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    self, Animation, Background, Color, Event, Layout, Length, Pixels, Point, Rectangle, Shell,
    Size, Theme, Widget,
};

use crate::core::Length::Fill;
use crate::core::widget::{Operation, operation};
use crate::operation::control::Control;
use iced_widget::slider;
use icedcn_core::Orientation;
use std::cmp::Ordering;

use std::ops::RangeInclusive;
use std::time::Duration;

/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// A [`Slider`] will try to fill the horizontal space of its container.
///
/// The [`Slider`] range of numeric values is generic and its step size defaults
/// to 1 unit.
///
/// Note: Under the hood values are converted to/from f64 so only values representable exactly as an f64
/// are possible to select via the slider. However it is likely that the precision of the slider at these
/// scales is already less than the precision lost from the f64 representation.
///
/// # Example
/// ```ignore
/// use icedcn::widget::slider;
///
/// struct State {
///    value: f32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ValueChanged(f32),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     slider(0.0..=100.0)
///         .value(state.value)
///         .on_value_change(Message::ValueChanged)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ValueChanged(value) => {
///             state.value = value;
///         }
///     }
/// }
/// ```
pub struct Slider<'a, T, Message, Theme = icedcn_core::Theme, Values = T>
where
    Theme: Catalog,
{
    range: RangeInclusive<T>,
    step: f64,
    shift_step: Option<f64>,
    value: Values,
    /// Whether the value is the app's (`true`) or kept in the widget tree.
    is_controlled: bool,
    default: Option<Values>,
    on_change: Option<Box<dyn Fn(Values) -> Message + 'a>>,
    on_release: Option<Message>,
    width: Length,
    height: Length,
    orientation: Orientation,
    class: Theme::Class<'a>,
    status: Option<Status>,
    pub(crate) is_disabled: bool,
}

impl<'a, T, Message, Theme> Slider<'a, T, Message, Theme>
where
    T: Copy + PartialOrd,
    Message: Clone,
    Theme: Catalog,
{
    /// The default height of a [`Slider`].
    pub const DEFAULT_HEIGHT: f32 = 16.0;

    /// Creates an uncontrolled [`Slider`] over `range`, starting at its
    /// start.
    pub fn new(range: RangeInclusive<T>) -> Self {
        let initial = *range.start();
        Slider::from_values(range, initial)
    }
}

/// Values associated with one slider. A scalar gives one thumb, a fixed-size
/// array gives a range with one focusable thumb per entry.
pub trait Thumbs<T>: Copy {
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];
}

impl<T: Copy> Thumbs<T> for T {
    fn values(&self) -> &[T] {
        std::slice::from_ref(self)
    }
    fn values_mut(&mut self) -> &mut [T] {
        std::slice::from_mut(self)
    }
}

impl<T: Copy, const N: usize> Thumbs<T> for [T; N] {
    fn values(&self) -> &[T] {
        self
    }
    fn values_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<'a, T, Message, Theme, Values> Slider<'a, T, Message, Theme, Values>
where
    T: Copy + PartialOrd,
    Values: Thumbs<T>,
    Message: Clone,
    Theme: Catalog,
{
    pub(super) fn from_values(range: RangeInclusive<T>, value: Values) -> Self {
        Slider {
            value,
            is_controlled: false,
            default: None,
            range,
            step: 1.0,
            shift_step: None,
            on_change: None,
            on_release: None,
            width: Fill,
            height: 16.into(),
            orientation: Orientation::Horizontal,
            class: Theme::default(),
            status: None,
            is_disabled: false,
        }
    }

    /// Controls the [`Slider`]: it always shows `value` (clamped to its
    /// range), and dragging only publishes
    /// [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: Values) -> Self {
        self.value = self.clamp(value);
        self.is_controlled = true;
        self
    }

    /// The initial value of an uncontrolled [`Slider`]. Later rebuilds keep
    /// the value the user dragged to.
    pub fn default_value(mut self, value: Values) -> Self {
        self.value = self.clamp(value);
        self.is_controlled = false;
        self
    }

    /// Sets the message published with the new value while the [`Slider`]
    /// is dragged.
    pub fn on_value_change(mut self, on_change: impl Fn(Values) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets whether the [`Slider`] is disabled: it keeps showing its value,
    /// ignores input and dims to half opacity.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    fn clamp(&self, mut values: Values) -> Values {
        for value in values.values_mut().iter_mut() {
            if *value < *self.range.start() || (*value).partial_cmp(self.range.start()).is_none() {
                *value = *self.range.start();
            } else if *value > *self.range.end() {
                *value = *self.range.end();
            }
        }
        values
            .values_mut()
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        values
    }

    /// Set the track axis. A vertical track increases from bottom to top.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        let orientation = orientation.into();
        if self.orientation != orientation {
            std::mem::swap(&mut self.width, &mut self.height);
        }
        self.orientation = orientation;
        self
    }

    /// Sets the optional default value for the [`Slider`].
    ///
    /// If set, the [`Slider`] will reset to this value when ctrl-clicked or command-clicked.
    pub fn default(mut self, default: impl Into<Values>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Sets the release message of the [`Slider`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// Sets the width of the [`Slider`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Slider`].
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().0.into();
        self
    }

    /// Sets the step size of the [`Slider`].
    pub fn step(mut self, step: impl num_traits::AsPrimitive<f64>) -> Self {
        let step = step.as_();
        if step.is_finite() && step > 0.0 {
            self.step = step;
        }
        self
    }

    /// Sets the optional "shift" step for the [`Slider`].
    ///
    /// If set, this value is used as the step while the shift key is pressed.
    pub fn shift_step(mut self, shift_step: impl num_traits::AsPrimitive<f64>) -> Self {
        let step = shift_step.as_();
        self.shift_step = (step.is_finite() && step > 0.0).then_some(step);
        self
    }

    /// Sets the style of the [`Slider`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Slider`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<T, Message, Theme, Values> Meta for Slider<'_, T, Message, Theme, Values> where Theme: Catalog {}

impl<T, Message, Theme, Renderer, Values> Widget<Message, Theme, Renderer>
    for Slider<'_, T, Message, Theme, Values>
where
    T: Copy + PartialOrd + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive,
    Values: Thumbs<T>,
    Message: Clone,
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();

        if self.is_controlled {
            state.uncontrolled.clear();
        } else if state.uncontrolled.len() == self.value.values().len() {
            for (to, from) in self.value.values_mut().iter_mut().zip(&state.uncontrolled) {
                if let Some(value) = T::from_f64(*from) {
                    *to = value;
                }
            }
            self.value = self.clamp(self.value);
        }
        state
            .focus
            .resize_with(self.value.values().len(), Focus::default);
        if self.is_disabled {
            state.dragging = None;
            state.focus.fill(Focus::default());
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, self.width, self.height);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.is_disabled {
            return;
        }
        let state = tree.state.downcast_mut::<State>();
        operation.custom(None, layout.bounds(), &mut state.control);
        for focus in &mut state.focus {
            operation.focusable(None, layout.bounds(), focus);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let start = (*self.range.start()).as_();
        let end = (*self.range.end()).as_().max(start);
        let step = if state.keyboard_modifiers.shift() {
            self.shift_step.unwrap_or(self.step)
        } else {
            self.step
        };
        let hovered = cursor.is_over(bounds) && cursor.is_over(*viewport);
        let mut next = self.value;

        if !self.is_disabled && !state.focus.is_empty() {
            if state.control.take_activation() {
                state.focus[0].0 = true;
            }
            let focus = state.focus.iter().position(|f| f.0);
            let at = |point: Point| {
                let (offset, extent) = match self.orientation {
                    Orientation::Horizontal => (point.x - bounds.x, bounds.width),
                    Orientation::Vertical => (bounds.y + bounds.height - point.y, bounds.height),
                };
                let cross = match self.orientation {
                    Orientation::Horizontal => bounds.height,
                    Orientation::Vertical => bounds.width,
                };
                let pad = (cross * 0.5).min(extent * 0.5);
                let fraction = ((offset - pad) / (extent - 2.0 * pad).max(1.0)).clamp(0.0, 1.0);
                if fraction == 1.0 {
                    end
                } else {
                    (start + ((f64::from(fraction) * (end - start)) / step).round() * step)
                        .clamp(start, end)
                }
            };
            let press = match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    Some((cursor.position(), Pointer::Mouse))
                }
                Event::Touch(touch::Event::FingerPressed { id, position, .. }) => {
                    Some((Some(*position), Pointer::Touch(*id)))
                }
                _ => None,
            };
            let mut target = None;
            if let Some((point, pointer)) = press {
                if state.dragging.is_none() {
                    let point = point.filter(|p| bounds.contains(*p) && viewport.contains(*p));
                    state.focus.fill(Focus::default());
                    if let Some(point) = point {
                        let candidate = at(point);
                        let index = closest(self.value.values(), candidate, focus);
                        state.focus[index].0 = true;
                        if state.keyboard_modifiers.command() {
                            if let Some(default) = self.default {
                                next = self.clamp(default);
                            }
                        } else {
                            state.dragging = Some((index, pointer));
                            target = Some((index, candidate));
                        }
                        shell.capture_event();
                    }
                }
            } else {
                match event {
                    Event::Mouse(mouse::Event::CursorMoved { position }) => {
                        if let Some((i, Pointer::Mouse)) = state.dragging {
                            target = Some((i, at(*position)));
                        }
                    }
                    Event::Touch(touch::Event::FingerMoved { id, position, .. }) => {
                        if let Some((i, Pointer::Touch(active))) = state.dragging
                            && active == *id
                        {
                            target = Some((i, at(*position)));
                        }
                    }
                    Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                    | Event::Touch(
                        touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. },
                    ) => {
                        let pointer = match event {
                            Event::Touch(
                                touch::Event::FingerLifted { id, .. }
                                | touch::Event::FingerLost { id, .. },
                            ) => Pointer::Touch(*id),
                            _ => Pointer::Mouse,
                        };
                        if state.dragging.is_some_and(|(_, active)| active == pointer) {
                            state.dragging = None;
                            if let Some(message) = &self.on_release {
                                shell.publish(message.clone());
                            }
                            shell.capture_event();
                        }
                    }
                    Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                        state.keyboard_modifiers = *modifiers
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: Key::Named(key),
                        ..
                    }) => {
                        if let Some(i) = focus {
                            let value = self.value.values()[i].as_();
                            let value = match key {
                                key::Named::ArrowUp | key::Named::ArrowRight => Some(value + step),
                                key::Named::ArrowDown | key::Named::ArrowLeft => Some(value - step),
                                key::Named::PageUp => Some(value + 10.0 * step),
                                key::Named::PageDown => Some(value - 10.0 * step),
                                key::Named::Home => Some(start),
                                key::Named::End => Some(end),
                                _ => None,
                            };
                            target = value.map(|v| (i, v));
                        }
                    }
                    Event::Mouse(mouse::Event::WheelScrolled { delta })
                        if hovered && state.keyboard_modifiers.control() =>
                    {
                        let (mouse::ScrollDelta::Lines { y, .. }
                        | mouse::ScrollDelta::Pixels { y, .. }) = delta;
                        let i = focus.unwrap_or(0);
                        target = Some((
                            i,
                            self.value.values()[i].as_() + step * f64::from(y.signum()),
                        ));
                    }
                    _ => {}
                }
            }
            if let Some((index, value)) = target {
                let values = self.value.values();
                let lower = index.checked_sub(1).map_or(start, |i| values[i].as_());
                let upper = values.get(index + 1).map_or(end, |v| v.as_());
                if let Some(value) = T::from_f64(value.clamp(lower, upper.max(lower))) {
                    next.values_mut()[index] = value;
                }
                shell.capture_event();
            }
            if next.values() != self.value.values() {
                if let Some(callback) = &self.on_change {
                    shell.publish(callback(next));
                }
                if !self.is_controlled {
                    self.value = next;
                    state.uncontrolled = next.values().iter().map(|v| v.as_()).collect();
                }
                shell.request_redraw();
            }
        }
        let current_status = if self.is_disabled {
            Status::Disabled
        } else if state.dragging.is_some() {
            Status::Dragged
        } else if hovered || state.focus.iter().any(|f| f.0) {
            Status::Hovered
        } else {
            Status::Active
        };
        let ring_visible = matches!(current_status, Status::Hovered | Status::Dragged);

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if ring_visible != state.ring.value() {
                state.ring.go_mut(ring_visible, *now);
            }

            if state.ring.is_animating(*now) {
                shell.request_redraw();
            }

            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();

        let current_status = self.status.unwrap_or(Status::Active);
        let mut style = theme.style(&self.class, current_status);

        let ring_progress = state
            .now
            .map(|now| state.ring.interpolate(0.0, 1.0, now))
            .unwrap_or_else(|| {
                if matches!(current_status, Status::Hovered | Status::Dragged) {
                    1.0
                } else {
                    0.0
                }
            });

        style.ring = if ring_progress > f32::EPSILON {
            style
                .ring
                .or_else(|| theme.style(&self.class, Status::Hovered).ring)
                .map(|ring| Ring {
                    color: ring.color.scale_alpha(ring_progress),
                    width: ring.width * ring_progress,
                })
        } else {
            None
        };

        let (handle_width, handle_height, handle_border_radius) = match style.handle.shape {
            HandleShape::Circle { radius } => (radius * 2.0, radius * 2.0, radius.into()),
            HandleShape::Rectangle {
                width,
                border_radius,
            } => (
                f32::from(width),
                bounds.width.min(bounds.height),
                border_radius,
            ),
        };

        let (start, end) = ((*self.range.start()).as_(), (*self.range.end()).as_());
        let fraction = |v: T| {
            if end > start {
                ((v.as_() - start) / (end - start)).clamp(0.0, 1.0) as f32
            } else {
                0.0
            }
        };
        let vertical = self.orientation == Orientation::Vertical;
        let point = |v: T| {
            let f = fraction(v);
            if vertical {
                Point::new(
                    bounds.center_x(),
                    bounds.y
                        + handle_height / 2.0
                        + (1.0 - f) * (bounds.height - handle_height).max(0.0),
                )
            } else {
                Point::new(
                    bounds.x + handle_width / 2.0 + f * (bounds.width - handle_width).max(0.0),
                    bounds.center_y(),
                )
            }
        };
        let rail = if vertical {
            Rectangle {
                x: bounds.center_x() - style.rail.width / 2.0,
                width: style.rail.width,
                ..bounds
            }
        } else {
            Rectangle {
                y: bounds.center_y() - style.rail.width / 2.0,
                height: style.rail.width,
                ..bounds
            }
        };
        let quad = renderer::Quad {
            bounds: rail,
            border: style.rail.border,
            ..renderer::Quad::default()
        };
        renderer.fill_quad(quad, style.rail.backgrounds.1);
        let values = self.value.values();
        let (Some(&first), Some(&last)) = (values.first(), values.last()) else {
            return;
        };
        let from = if values.len() == 1 {
            if vertical {
                Point::new(rail.center_x(), rail.y + rail.height)
            } else {
                Point::new(rail.x, rail.center_y())
            }
        } else {
            point(first)
        };
        let to = point(last);
        let filled = if vertical {
            Rectangle {
                y: to.y,
                height: (from.y - to.y).max(0.0),
                ..rail
            }
        } else {
            Rectangle {
                x: from.x,
                width: (to.x - from.x).max(0.0),
                ..rail
            }
        };
        renderer.fill_quad(
            renderer::Quad {
                bounds: filled,
                ..quad
            },
            style.rail.backgrounds.0,
        );
        for (i, value) in values.iter().copied().enumerate() {
            let center = point(value);
            let handle_bounds = Rectangle {
                x: center.x - handle_width / 2.0,
                y: center.y - handle_height / 2.0,
                width: handle_width,
                height: handle_height,
            };
            if let Some(ring) = style.ring
                && ring.width > 0.0
                && (values.len() == 1
                    || state.focus.get(i).is_some_and(|f| f.0)
                    || _cursor.is_over(handle_bounds))
            {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: handle_bounds.x - ring.width,
                            y: handle_bounds.y - ring.width,
                            width: handle_bounds.width + ring.width * 2.0,
                            height: handle_bounds.height + ring.width * 2.0,
                        },
                        border: Border {
                            radius: expand_radius(handle_border_radius, ring.width),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(ring.color),
                );
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds: handle_bounds,
                    border: Border {
                        radius: handle_border_radius,
                        width: style.handle.border_width,
                        color: style.handle.border_color,
                    },
                    ..renderer::Quad::default()
                },
                style.handle.background,
            );
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.is_disabled && cursor.is_over(layout.bounds()) {
            mouse::Interaction::NotAllowed
        } else {
            mouse::Interaction::default()
        }
    }
}

fn closest<T: num_traits::AsPrimitive<f64>>(
    values: &[T],
    value: f64,
    focused: Option<usize>,
) -> usize {
    let distance = |i: usize| (values[i].as_() - value).abs();
    let mut best = focused.unwrap_or(0);
    for i in 0..values.len() {
        if distance(i) < distance(best)
            || (distance(i) == distance(best)
                && values[i].as_() == values[best].as_()
                && ((value > values[i].as_() && i > best) || (value < values[i].as_() && i < best)))
        {
            best = i;
        }
    }
    best
}

#[derive(Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Touch(touch::Finger),
}

#[derive(Default, Clone)]
struct Focus(bool);
impl operation::Focusable for Focus {
    fn is_focused(&self) -> bool {
        self.0
    }
    fn focus(&mut self) {
        self.0 = true;
    }
    fn unfocus(&mut self) {
        self.0 = false;
    }
}

struct State {
    /// The value of an uncontrolled slider, kept across rebuilds.
    uncontrolled: Vec<f64>,
    dragging: Option<(usize, Pointer)>,
    focus: Vec<Focus>,
    control: Control,
    keyboard_modifiers: keyboard::Modifiers,
    ring: Animation<bool>,
    now: Option<Instant>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            uncontrolled: Vec::new(),
            dragging: None,
            focus: Vec::new(),
            control: Control::new(false),
            keyboard_modifiers: keyboard::Modifiers::default(),
            ring: Animation::new(false)
                .easing(Easing::Custom(|t| {
                    crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
                }))
                .duration(Duration::from_millis(150)),
            now: None,
        }
    }
}

/// The possible status of a [`Slider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Slider`] can be interacted with.
    Active,
    /// The [`Slider`] is being hovered.
    Hovered,
    /// The [`Slider`] is being dragged.
    Dragged,
    /// The [`Slider`] cannot be interacted with.
    Disabled,
}

/// The appearance of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The colors of the rail of the slider.
    pub rail: Rail,
    /// The appearance of the [`Handle`] of the slider.
    pub handle: Handle,
    /// The hover/focus ring painted around the [`Handle`].
    pub ring: Option<Ring>,
}

impl Style {
    /// Changes the [`HandleShape`] of the [`Style`] to a circle
    /// with the given radius.
    pub fn with_circular_handle(mut self, radius: impl Into<Pixels>) -> Self {
        self.handle.shape = HandleShape::Circle {
            radius: radius.into().0,
        };
        self
    }
}

/// The appearance of a slider rail
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rail {
    /// The backgrounds of the rail of the slider.
    pub backgrounds: (Background, Background),
    /// The width of the stroke of a slider rail.
    pub width: f32,
    /// The border of the rail.
    pub border: Border,
}

/// The appearance of the handle of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Handle {
    /// The shape of the handle.
    pub shape: HandleShape,
    /// The [`Background`] of the handle.
    pub background: Background,
    /// The border width of the handle.
    pub border_width: f32,
    /// The border [`Color`] of the handle.
    pub border_color: Color,
}

/// The appearance of the ring around the handle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    /// The ring color.
    pub color: Color,
    /// The ring width in logical pixels.
    pub width: f32,
}

/// The shape of the handle of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandleShape {
    /// A circular handle.
    Circle {
        /// The radius of the circle.
        radius: f32,
    },
    /// A rectangular shape.
    Rectangle {
        /// The width of the rectangle.
        width: u16,
        /// The border radius of the corners of the rectangle.
        border_radius: border::Radius,
    },
}

/// The theme catalog of a [`Slider`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Slider`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

impl From<slider::Style> for Style {
    fn from(style: slider::Style) -> Self {
        Self {
            rail: Rail {
                backgrounds: style.rail.backgrounds,
                width: style.rail.width,
                border: style.rail.border,
            },
            handle: Handle {
                shape: match style.handle.shape {
                    slider::HandleShape::Circle { radius } => HandleShape::Circle { radius },
                    slider::HandleShape::Rectangle {
                        width,
                        border_radius,
                    } => HandleShape::Rectangle {
                        width,
                        border_radius,
                    },
                },
                background: style.handle.background,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            },
            ring: None,
        }
    }
}

impl From<Style> for slider::Style {
    fn from(style: Style) -> Self {
        Self {
            rail: slider::Rail {
                backgrounds: style.rail.backgrounds,
                width: style.rail.width,
                border: style.rail.border,
            },
            handle: slider::Handle {
                shape: match style.handle.shape {
                    HandleShape::Circle { radius } => slider::HandleShape::Circle { radius },
                    HandleShape::Rectangle {
                        width,
                        border_radius,
                    } => slider::HandleShape::Rectangle {
                        width,
                        border_radius,
                    },
                },
                background: style.handle.background,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            },
        }
    }
}

impl From<Status> for slider::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Active => slider::Status::Active,
            Status::Hovered => slider::Status::Hovered,
            Status::Dragged => slider::Status::Dragged,
            Status::Disabled => slider::Status::Active,
        }
    }
}

/// The default style of a [`Slider`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let color = match status {
        Status::Active | Status::Disabled => palette.primary.base.color,
        Status::Hovered => palette.primary.strong.color,
        Status::Dragged => palette.primary.weak.color,
    };

    Style {
        rail: Rail {
            backgrounds: (color.into(), palette.background.strong.color.into()),
            width: 4.0,
            border: Border {
                radius: 2.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 7.0 },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
        ring: None,
    }
}

fn expand_radius(radius: border::Radius, by: f32) -> border::Radius {
    border::Radius {
        top_left: radius.top_left + by,
        top_right: radius.top_right + by,
        bottom_right: radius.bottom_right + by,
        bottom_left: radius.bottom_left + by,
    }
}
