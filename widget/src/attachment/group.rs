use std::any::Any;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use iced_core::Length::Fill;
use iced_core::gradient::Linear;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::time::Instant;
use iced_core::widget::operation::{Animation, Focusable, Scrollable, scrollable::AbsoluteOffset};
use iced_core::widget::{Id, Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, Widget, layout,
    mouse, overlay, renderer, window,
};
use iced_widget::{Renderer, Widget as _};
use icedcn_core::theme::scrollable;
use icedcn_core::{Color, Theme};

use super::surface;
use crate::scrollable::Direction;
use crate::{Compose, Inherited, Row, Slot, container, scroll_area};

/// A keyboard-accessible, snapping strip of attachment cards.
pub struct Group<'a, Message> {
    cards: Vec<Slot<'a, Message>>,
    background: fn(&Theme) -> Color,
}
pub fn group<'a, Message>(
    cards: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Group<'a, Message> {
    Group {
        cards: cards.into_iter().collect(),
        background: |theme| theme.neutral.base,
    }
}
impl<Message> Group<'_, Message> {
    /// The uniform canvas behind the strip, used for its scroll-edge mask.
    pub fn fade_to(mut self, background: fn(&Theme) -> Color) -> Self {
        self.background = background;
        self
    }
}
impl<'a, Message: 'a> Compose<Message> for Group<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for card in &mut self.cards {
            card.inherit(from);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let content =
            scroll_area(container(Row::with_children(self.cards).spacing(12)).padding([4, 0]))
                .direction(Direction::Horizontal(scroll_area::bar()))
                .focus_group()
                .style(scrollable::hidden)
                .width(Fill)
                .into();
        Viewport {
            content,
            background: self.background,
        }
        .boxed()
    }
}
impl<'a, Message: 'a> From<Group<'a, Message>> for Slot<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        Slot::new(group)
    }
}
struct Viewport<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    background: fn(&Theme) -> Color,
}
#[derive(Default)]
struct State {
    pending: Option<Instant>,
    x: f32,
    end: f32,
    start: f32,
    direction: i8,
}
impl<Message> Meta for Viewport<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Viewport<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        op: &mut dyn Operation,
    ) {
        if let Some((child, state)) = layout.iter_mut(&mut tree.children).next() {
            self.content.operate(state, child, viewport, renderer, op);
        }
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Some((child, child_state)) = layout.iter_mut(&mut tree.children).next() else {
            return;
        };
        self.content
            .update(child_state, event, child, cursor, renderer, shell, viewport);
        let mut measured = Measure::default();
        self.content
            .operate(child_state, child, viewport, renderer, &mut measured);
        let state = tree.state.downcast_mut::<State>();
        let previous = state.x;
        state.x = measured.x;
        state.end = measured.end;
        // Browsers scroll an ancestor when a focused action leaves an arrow
        // unhandled. Preserve controls that consume their own arrow keys.
        if measured.focused
            && !shell.is_event_captured()
            && matches!(
                event,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::ArrowLeft | Named::ArrowRight),
                    ..
                })
            )
        {
            shell.capture_event();
        }
        let moved = matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. }))
            && cursor.is_over(layout.bounds());
        let keyboard = matches!(
            event,
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowLeft | Named::ArrowRight | Named::Home | Named::End),
                ..
            })
        ) && shell.is_event_captured();
        if moved || keyboard {
            let direction = match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::ArrowRight),
                    ..
                }) => 1,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::ArrowLeft),
                    ..
                }) => -1,
                Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    let (x, y) = match delta {
                        mouse::ScrollDelta::Pixels { x, y }
                        | mouse::ScrollDelta::Lines { x, y } => (*x, *y),
                    };
                    -(if x != 0. { x } else { y }).signum() as i8
                }
                _ => 0,
            };
            if state.pending.is_none() || state.direction != direction {
                state.start = previous;
            }
            state.direction = direction;
            state.pending = Some(Instant::now() + Duration::from_millis(120));
            shell.request_redraw();
        }
        if let (Some(at), Event::Window(window::Event::RedrawRequested(now))) =
            (state.pending, event)
        {
            if *now >= at {
                let target = measured
                    .cards
                    .iter()
                    .map(|x| (x - measured.origin - 4.).clamp(0., measured.end))
                    .chain([0., measured.end])
                    .filter(|target| match state.direction {
                        1 => *target > state.start + 0.5,
                        -1 => *target < state.start - 0.5,
                        _ => true,
                    })
                    .min_by(|a, b| (a - measured.x).abs().total_cmp(&(b - measured.x).abs()));
                if let Some(target) = target {
                    self.content
                        .operate(child_state, child, viewport, renderer, &mut Snap(target));
                    shell.request_redraw();
                }
                state.pending = None;
            } else {
                shell.request_redraw();
            }
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
        let Some((child, state)) = layout.iter(&tree.children).next() else {
            return;
        };
        self.content
            .draw(state, renderer, theme, style, child, cursor, viewport);
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let length = (bounds.width * 0.12).min(40.);
        if let Some(clip) = bounds.intersection(viewport) {
            let color: iced_core::Color = (self.background)(theme).into();
            for (visible, x, start, end) in [
                (state.x > 0.5, bounds.x, color, color.scale_alpha(0.)),
                (
                    state.end - state.x > 0.5,
                    bounds.x + bounds.width - length,
                    color.scale_alpha(0.),
                    color,
                ),
            ] {
                if visible {
                    renderer.with_layer(clip, |renderer| {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: Rectangle {
                                    x,
                                    width: length,
                                    ..bounds
                                },
                                ..renderer::Quad::default()
                            },
                            Linear::new(FRAC_PI_2).add_stop(0., start).add_stop(1., end),
                        )
                    });
                }
            }
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
        layout
            .iter(&tree.children)
            .next()
            .map_or(mouse::Interaction::None, |(child, state)| {
                self.content
                    .mouse_interaction(state, child, cursor, viewport, renderer)
            })
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        if let Some((child, state)) = layout.iter_mut(&mut tree.children).next() {
            self.content
                .overlay(state, child, renderer, viewport, translation, window)
        } else {
            Vec::new()
        }
    }
}
#[derive(Default)]
struct Measure {
    origin: f32,
    x: f32,
    end: f32,
    cards: Vec<f32>,
    focused: bool,
}
impl Operation for Measure {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        f(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.origin = bounds.x;
        self.x = offset.x;
        self.end = (content.width - bounds.width).max(0.);
    }
    fn custom(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if state.is::<surface::State>() {
            self.cards.push(bounds.x);
        }
    }
    fn focusable(&mut self, _: Option<&Id>, _: Rectangle, state: &mut dyn Focusable) {
        self.focused |= state.is_focused();
    }
}
struct Snap(f32);
impl Operation for Snap {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        f(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        _: Vector,
        state: &mut dyn Scrollable,
    ) {
        state.scroll_to(
            AbsoluteOffset {
                x: Some(self.0),
                y: None,
            },
            Animation::Smooth,
            bounds,
            content,
        );
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(group: Group<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}
