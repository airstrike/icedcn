use std::time::Duration;

use iced_core::keyboard::{self, Key, key::Named};
use iced_core::time::Instant;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Point, Rectangle, Renderer as _, Shell, Size, Vector, Widget,
    layout, mouse, overlay, renderer, touch, window,
};
use iced_widget::Renderer;
use icedcn_core::{Orientation, Theme};

use crate::operation::focus;

const GUTTER: f32 = 48.0;
const CONTROL: f32 = 28.0;
const STEP: f32 = 1.0 / 60.0;

pub(super) struct Viewport<'a, Message> {
    pub children: Vec<Element<'a, super::Event<Message>, Theme, Renderer>>,
    pub count: usize,
    pub visible: usize,
    pub width: Length,
    pub height: Option<f32>,
    pub axis: Orientation,
    pub index: usize,
    pub snaps: Vec<f32>,
    pub looping: bool,
    pub direction: i8,
    pub gap: f32,
    pub autoplay: Option<Duration>,
    pub stop_on_hover: bool,
    pub disabled: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Touch(touch::Finger),
}

struct Drag {
    pointer: Pointer,
    origin: Point,
    offset: f32,
    moved: bool,
}

#[derive(Default)]
struct State {
    selected: Option<usize>,
    focused_slide: Option<usize>,
    position: f32,
    target: f32,
    velocity: f32,
    frame: Option<Instant>,
    remainder: f32,
    track: Rectangle,
    stride: f32,
    drag: Option<Drag>,
    hover: bool,
    stopped: bool,
    due: Option<Instant>,
}

impl State {
    fn advance(&mut self, now: Instant) -> bool {
        let elapsed = self.frame.replace(now).map_or(STEP, |last| {
            now.saturating_duration_since(last).as_secs_f32().min(1.0)
        });
        if self.drag.is_some() {
            return false;
        }
        self.remainder += elapsed;
        while self.remainder >= STEP {
            // Advance the damped spring in fixed steps of 1/60 second.
            self.velocity = (self.velocity + (self.target - self.position) / 25.0) * 0.68;
            self.position += self.velocity;
            self.remainder -= STEP;
        }
        if (self.position - self.target).abs() * self.stride < 0.001 {
            self.position = self.target;
            self.velocity = 0.0;
            self.remainder = 0.0;
            false
        } else {
            true
        }
    }

    fn stop(&mut self) {
        self.stopped = true;
        self.due = None;
    }
}

impl<Message> Viewport<'_, Message> {
    fn vertical(&self) -> bool {
        self.axis == Orientation::Vertical
    }

    fn axis(&self, point: Vector) -> (f32, f32) {
        if self.vertical() {
            (point.y, point.x)
        } else {
            (point.x, point.y)
        }
    }

    fn track(&self, state: &State, layout: Layout) -> Rectangle {
        state.track + Vector::new(layout.position().x, layout.position().y)
    }

    fn child_layout(&self, index: usize, tree: &Tree, layout: Layout) -> Layout {
        let state = tree.state.downcast_ref::<State>();
        let child = &tree.children[index];
        let mut offset = child.translation;
        if index < self.count {
            let major = index as f32 * state.stride - state.position * state.stride;
            let major = if self.looping {
                let cell = (state.stride - self.gap).max(0.0);
                (major + cell).rem_euclid(self.count as f32 * state.stride) - cell
            } else {
                major
            };
            if self.vertical() {
                offset.y = state.track.y + major;
            } else {
                offset.x = state.track.x + major;
            }
        }
        Layout::new(child.size).move_to(layout.position() + offset)
    }

    fn focused(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> (bool, Option<usize>) {
        let mut any = false;
        let mut slide = None;
        for index in 0..self.children.len() {
            let child_layout = self.child_layout(index, tree, layout);
            let count = focus::count(|operation| {
                self.children[index].operate(
                    &mut tree.children[index],
                    child_layout,
                    viewport,
                    renderer,
                    operation,
                );
            });
            if count.focused.is_some() {
                any = true;
                if index < self.count {
                    slide = Some(index);
                }
            }
        }
        (any, slide)
    }

    fn nearest(&self, position: f32) -> usize {
        self.snaps
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let distance = |snap: f32| {
                    let delta = snap - position;
                    if self.looping {
                        let cycle = self.count as f32;
                        (delta - (delta / cycle).round() * cycle).abs()
                    } else {
                        delta.abs()
                    }
                };
                distance(**a).total_cmp(&distance(**b))
            })
            .map_or(0, |(index, _)| index)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "forwards iced's Widget::update context to children"
    )]
    fn dispatch(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, super::Event<Message>>,
        viewport: &Rectangle,
        dragging: bool,
    ) {
        let clip = self
            .track(tree.state.downcast_ref::<State>(), layout)
            .intersection(viewport);
        // Controls remain outside the track clip and see every event, including cancellation.
        for index in (self.count..self.children.len()).chain(0..self.count) {
            let child_layout = self.child_layout(index, tree, layout);
            let child_viewport = if index < self.count {
                clip.unwrap_or_default()
            } else {
                *viewport
            };
            let child_cursor =
                if index < self.count && (dragging || !cursor.is_over(child_viewport)) {
                    mouse::Cursor::Unavailable
                } else {
                    cursor
                };
            self.children[index].update(
                &mut tree.children[index],
                event,
                child_layout,
                child_cursor,
                renderer,
                shell,
                &child_viewport,
            );
        }
    }
}

impl<Message> Meta for Viewport<'_, Message> {}

impl<Message> Widget<super::Event<Message>, Theme, Renderer> for Viewport<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
        let state = tree.state.downcast_mut::<State>();
        let mut target = self.snaps.get(self.index).copied().unwrap_or(0.0);
        if self.looping && state.selected.is_some() {
            let cycle = self.count as f32;
            target += ((state.target - target) / cycle).round() * cycle;
            if state.selected != Some(self.index) {
                if self.direction > 0 && target <= state.target {
                    target += cycle;
                } else if self.direction < 0 && target >= state.target {
                    target -= cycle;
                }
            }
        }
        if state.selected.is_none() {
            state.position = target;
        }
        state.selected = Some(self.index);
        if state.drag.is_none() {
            if state.target != target
                && (state.position - state.target).abs() * state.stride < 0.001
            {
                // An idle interval is not elapsed animation time for a new movement.
                state.frame = None;
                state.remainder = 0.0;
            }
            state.target = target;
        }
        if self.disabled {
            state.drag = None;
            state.due = None;
        }
    }

    fn size(&self) -> Size<Length> {
        let width = match self.width {
            Length::Fixed(width) if !self.vertical() => Length::Fixed(width + 2.0 * GUTTER),
            width => width,
        };
        Size::new(width, Length::Shrink)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let vertical = self.vertical();
        let horizontal_gutters = if vertical { 0.0 } else { 2.0 * GUTTER };
        let available = (limits.max.width - horizontal_gutters).max(0.0);
        let width = match self.width {
            Length::Fixed(width) => width.min(available),
            _ if available.is_finite() => available,
            _ => 320.0,
        };
        let height = self.height.map(|height| {
            height.min((limits.max.height - if vertical { 2.0 * GUTTER } else { 0.0 }).max(0.0))
        });
        let cell_width = if vertical {
            width
        } else {
            ((width + self.gap) / self.visible as f32 - self.gap).max(0.0)
        };
        let cell_height = if vertical {
            height.map(|height| ((height + self.gap) / self.visible as f32 - self.gap).max(0.0))
        } else {
            height
        };
        let mut natural_height: f32 = 0.0;
        for (slide, child) in self.children[..self.count]
            .iter_mut()
            .zip(&mut tree.children[..self.count])
        {
            let child_limits = layout::Limits::new(
                Size::ZERO,
                Size::new(cell_width, cell_height.unwrap_or(f32::INFINITY)),
            )
            .width(cell_width);
            slide.layout(child, renderer, &child_limits);
            natural_height = natural_height.max(child.size.height);
        }
        let height = height.unwrap_or(if vertical {
            natural_height * self.visible as f32 + self.gap * self.visible.saturating_sub(1) as f32
        } else {
            natural_height
        });
        let track = Rectangle {
            x: if vertical { 0.0 } else { GUTTER },
            y: if vertical { GUTTER } else { 0.0 },
            width,
            height,
        };
        let stride = ((if vertical { height } else { width }) + self.gap) / self.visible as f32;
        for (index, child) in tree.children[..self.count].iter_mut().enumerate() {
            child.translation = Vector::new(track.x, track.y)
                + if vertical {
                    Vector::new(0.0, index as f32 * stride)
                } else {
                    Vector::new(index as f32 * stride, 0.0)
                };
        }
        for index in self.count..self.children.len() {
            let child = &mut tree.children[index];
            self.children[index].layout(
                child,
                renderer,
                &layout::Limits::new(Size::ZERO, Size::new(CONTROL, CONTROL)),
            );
            let next = index == self.count + 1;
            child.translation = if vertical {
                Vector::new(
                    (width - CONTROL) / 2.0,
                    if next {
                        height + 2.0 * GUTTER - CONTROL
                    } else {
                        0.0
                    },
                )
            } else {
                Vector::new(
                    if next {
                        width + 2.0 * GUTTER - CONTROL
                    } else {
                        0.0
                    },
                    (height - CONTROL) / 2.0,
                )
            };
        }
        tree.size = Size::new(
            width + horizontal_gutters,
            height + if vertical { 2.0 * GUTTER } else { 0.0 },
        );
        let state = tree.state.downcast_mut::<State>();
        state.track = track;
        state.stride = stride.max(f32::EPSILON);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        let clip = self
            .track(tree.state.downcast_ref::<State>(), layout)
            .intersection(viewport)
            .unwrap_or_default();
        operation.traverse(&mut |operation| {
            for index in 0..self.children.len() {
                let child_layout = self.child_layout(index, tree, layout);
                self.children[index].operate(
                    &mut tree.children[index],
                    child_layout,
                    if index < self.count { &clip } else { viewport },
                    renderer,
                    operation,
                );
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, super::Event<Message>>,
        viewport: &Rectangle,
    ) {
        let track = self.track(tree.state.downcast_ref::<State>(), layout);
        if self.disabled {
            self.dispatch(
                tree, event, layout, cursor, renderer, shell, viewport, false,
            );
            return;
        }
        let (focused, slide_focus) = self.focused(tree, layout, viewport, renderer);
        let state = tree.state.downcast_mut::<State>();
        let previous_focus = std::mem::replace(&mut state.focused_slide, slide_focus);
        if let Some(index) = slide_focus
            && previous_focus != Some(index)
        {
            let child_layout = self.child_layout(index, tree, layout);
            let bounds = child_layout.bounds();
            tree.state.downcast_mut::<State>().stop();
            let clipped = if self.vertical() {
                bounds.y < track.y - 0.5 || bounds.y + bounds.height > track.y + track.height + 0.5
            } else {
                bounds.x < track.x - 0.5 || bounds.x + bounds.width > track.x + track.width + 0.5
            };
            if clipped {
                let snap = self.nearest(index as f32);
                if snap != self.index {
                    shell.publish(super::Event::Select(snap));
                }
            }
        }
        let state = tree.state.downcast_mut::<State>();
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            ..
        }) = event
            && focused
        {
            let action = match key {
                Named::ArrowLeft => Some(super::Event::Previous),
                Named::ArrowRight => Some(super::Event::Next),
                _ => None,
            };
            if let Some(action) = action {
                shell.publish(action);
                shell.capture_event();
                shell.request_redraw();
                return;
            }
        }
        let hit = |point| track.contains(point) && viewport.contains(point);
        let press = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                cursor.position().map(|point| (Pointer::Mouse, point))
            }
            Event::Touch(touch::Event::FingerPressed { id, position, .. }) => {
                Some((Pointer::Touch(*id), *position))
            }
            _ => None,
        };
        if let Some((pointer, position)) = press
            && state.drag.is_none()
            && hit(position)
        {
            state.stop();
            state.drag = Some(Drag {
                pointer,
                origin: position,
                offset: state.position,
                moved: false,
            });
            state.target = state.position;
            state.velocity = 0.0;
        }
        let movement = match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some((Pointer::Mouse, *position))
            }
            Event::Touch(touch::Event::FingerMoved { id, position, .. }) => {
                Some((Pointer::Touch(*id), *position))
            }
            _ => None,
        };
        if let Some((pointer, position)) = movement
            && let Some(drag) = &mut state.drag
            && drag.pointer == pointer
        {
            let (along, cross) = self.axis(position - drag.origin);
            if !drag.moved && cross.abs() > along.abs() && cross.abs() > 10.0 {
                state.drag = None;
                state.target = self.snaps.get(self.index).copied().unwrap_or(0.0);
            } else if along.abs() > 10.0 || drag.moved {
                drag.moved = true;
                let requested = drag.offset - along / state.stride;
                let end = self.snaps.last().copied().unwrap_or(0.0);
                state.position = if self.looping {
                    requested
                } else {
                    let clamped = requested.clamp(0.0, end);
                    clamped + (requested - clamped) * 0.35
                };
                state.target = state.position;
                shell.capture_event();
                shell.request_redraw();
            }
        }
        let release = match event {
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                cursor.position().map(|position| (Pointer::Mouse, position))
            }
            Event::Touch(touch::Event::FingerLifted { id, position, .. }) => {
                Some((Pointer::Touch(*id), *position))
            }
            _ => None,
        };
        let mut dragged = state.drag.as_ref().is_some_and(|drag| drag.moved);
        if let Some((pointer, position)) = release
            && state
                .drag
                .as_ref()
                .is_some_and(|drag| drag.pointer == pointer)
            && let Some(drag) = state.drag.take()
        {
            state.frame = None;
            state.remainder = 0.0;
            let (along, cross) = self.axis(position - drag.origin);
            dragged = drag.moved || along.abs() > 35.0 && along.abs() > cross.abs();
            if dragged {
                let wanted = drag.offset - along / state.stride;
                let mut index = self.nearest(wanted);
                if index == self.index && along.abs() > 35.0 {
                    let count = self.snaps.len();
                    index = if along < 0.0 {
                        if self.looping {
                            (self.index + 1) % count
                        } else {
                            (self.index + 1).min(count.saturating_sub(1))
                        }
                    } else if self.index == 0 && self.looping {
                        count - 1
                    } else {
                        self.index.saturating_sub(1)
                    };
                }
                // Controlled callers may decline the request: settle back unless diff accepts it.
                state.target = self.snaps.get(self.index).copied().unwrap_or(0.0);
                if self.looping {
                    let cycle = self.count as f32;
                    state.target += ((state.position - state.target) / cycle).round() * cycle;
                }
                shell.publish(super::Event::Select(index));
                shell.request_redraw();
            } else {
                state.target = self.snaps.get(self.index).copied().unwrap_or(0.0);
            }
        }
        if let Event::Touch(touch::Event::FingerLost { id, .. }) = event
            && state
                .drag
                .as_ref()
                .is_some_and(|drag| drag.pointer == Pointer::Touch(*id))
        {
            state.drag = None;
            state.frame = None;
            state.remainder = 0.0;
            state.target = self.snaps.get(self.index).copied().unwrap_or(0.0);
            shell.request_redraw();
        }
        let hover = cursor
            .position()
            .is_some_and(|point| layout.bounds().contains(point) && viewport.contains(point));
        if hover != state.hover {
            state.hover = hover;
            state.due = None;
            if hover && self.stop_on_hover {
                state.stop();
            }
            shell.request_redraw();
        }
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            if state.advance(*now) && track.intersects(viewport) {
                shell.request_redraw();
            }
            if let Some(delay) = self.autoplay
                && !state.stopped
                && !state.hover
                && layout.bounds().intersects(viewport)
            {
                let due = state.due.get_or_insert(*now + delay);
                if *now >= *due {
                    shell.publish(super::Event::Advance);
                    *due = *now + delay;
                }
                shell.request_redraw_at(*due);
            } else {
                state.due = None;
            }
        } else if (state.target - state.position).abs() > f32::EPSILON {
            shell.request_redraw();
        }
        self.dispatch(
            tree, event, layout, cursor, renderer, shell, viewport, dragged,
        );
        if dragged {
            shell.capture_event();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let track = self.track(tree.state.downcast_ref::<State>(), layout);
        if let Some(clip) = track.intersection(viewport) {
            renderer.with_layer(clip, |renderer| {
                for index in 0..self.count {
                    let child_layout = self.child_layout(index, tree, layout);
                    if child_layout.bounds().intersects(&clip) {
                        self.children[index].draw(
                            &tree.children[index],
                            renderer,
                            theme,
                            style,
                            child_layout,
                            if cursor.is_over(clip) {
                                cursor
                            } else {
                                mouse::Cursor::Unavailable
                            },
                            &clip,
                        );
                    }
                }
            });
        }
        for index in self.count..self.children.len() {
            self.children[index].draw(
                &tree.children[index],
                renderer,
                theme,
                style,
                self.child_layout(index, tree, layout),
                cursor,
                viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        if state.drag.as_ref().is_some_and(|drag| drag.moved) {
            return mouse::Interaction::Grabbing;
        }
        let track = self.track(state, layout);
        (0..self.children.len())
            .filter(|index| *index >= self.count || cursor.is_over(track))
            .map(|index| {
                self.children[index].mouse_interaction(
                    &tree.children[index],
                    self.child_layout(index, tree, layout),
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, super::Event<Message>, Theme, Renderer>> {
        let layouts = (0..self.children.len())
            .map(|index| self.child_layout(index, tree, layout))
            .collect::<Vec<_>>();
        let clip = self
            .track(tree.state.downcast_ref::<State>(), layout)
            .intersection(viewport)
            .unwrap_or_default();
        let mut overlays = Vec::new();
        for (index, ((child, child_tree), child_layout)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layouts)
            .enumerate()
        {
            let visible = if index < self.count { &clip } else { viewport };
            if child_layout.bounds().intersects(visible) {
                overlays.extend(child.overlay(
                    child_tree,
                    child_layout,
                    renderer,
                    viewport,
                    translation,
                    window,
                ));
            }
        }
        overlays
    }
}
