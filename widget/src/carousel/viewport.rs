use iced_core::keyboard::{self, Key, key::Named};
use iced_core::time::Instant;
use iced_core::widget::{Meta, Operation, Tree, operation::Focusable, tree};
use iced_core::{
    Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget, layout, mouse,
    overlay, renderer, touch, window,
};
use iced_widget::Renderer;
use icedcn_core::{Orientation, Theme};
use std::time::Duration;

pub(super) struct Viewport<'a, Message> {
    pub child: Element<'a, super::Event<Message>, Theme, Renderer>,
    pub axis: Orientation,
    pub autoplay: Option<Duration>,
    pub disabled: bool,
}
#[derive(Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Touch(touch::Finger),
}
#[derive(Default)]
struct State {
    press: Option<(Pointer, Point)>,
    focus: bool,
    hover: bool,
    due: Option<Instant>,
}
impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focus
    }
    fn focus(&mut self) {
        self.focus = true;
    }
    fn unfocus(&mut self) {
        self.focus = false;
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
        tree.diff_children(std::slice::from_mut(&mut self.child));
    }
    fn size(&self) -> Size<Length> {
        self.child.size()
    }
    fn layout(&mut self, tree: &mut Tree, r: &Renderer, limits: &layout::Limits) {
        let child = &mut tree.children[0];
        self.child.layout(child, r, limits);
        child.translation = Vector::ZERO;
        tree.size = child.size;
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        r: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if !self.disabled {
            operation.focusable(None, layout.bounds(), tree.state.downcast_mut::<State>());
        }
        if let Some((l, t)) = layout.iter_mut(&mut tree.children).next() {
            self.child.operate(t, l, viewport, r, operation);
        }
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        r: &Renderer,
        shell: &mut Shell<'_, super::Event<Message>>,
        viewport: &Rectangle,
    ) {
        let b = layout.bounds();
        let state = tree.state.downcast_mut::<State>();
        if !self.disabled {
            let hit = |p: Point| b.contains(p) && viewport.contains(p);
            let press = match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    cursor.position().map(|p| (Pointer::Mouse, p))
                }
                Event::Touch(touch::Event::FingerPressed { id, position }) => {
                    Some((Pointer::Touch(*id), *position))
                }
                _ => None,
            };
            if let Some((pointer, p)) = press {
                state.focus = hit(p);
                if state.focus {
                    state.press = Some((pointer, p));
                }
            }
            let release = match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    cursor.position().map(|p| (Pointer::Mouse, p))
                }
                Event::Touch(touch::Event::FingerLifted { id, position }) => {
                    Some((Pointer::Touch(*id), *position))
                }
                _ => None,
            };
            if let Some((pointer, p)) = release
                && let Some((active, start)) = state.press
                && active == pointer
            {
                state.press = None;
                let delta = p - start;
                let (along, cross) = match self.axis {
                    Orientation::Horizontal => (delta.x, delta.y),
                    Orientation::Vertical => (delta.y, delta.x),
                };
                if along.abs() > 35.0 && along.abs() > cross.abs() {
                    shell.publish(if along < 0.0 {
                        super::Event::Next
                    } else {
                        super::Event::Previous
                    });
                    shell.capture_event();
                    return;
                }
            }
            if let Event::Touch(touch::Event::FingerLost { id, .. }) = event
                && state.press.is_some_and(|(p, _)| p == Pointer::Touch(*id))
            {
                state.press = None;
            }
            if state.focus
                && let Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(k), ..
                }) = event
            {
                let next = match (self.axis, k) {
                    (_, Named::ArrowLeft) | (Orientation::Vertical, Named::ArrowUp) => {
                        Some(super::Event::Previous)
                    }
                    (_, Named::ArrowRight) | (Orientation::Vertical, Named::ArrowDown) => {
                        Some(super::Event::Next)
                    }
                    _ => None,
                };
                if let Some(next) = next {
                    shell.publish(next);
                    shell.capture_event();
                    return;
                }
            }
            if let Some(delay) = self.autoplay {
                let hover = cursor.position().is_some_and(hit);
                if hover != state.hover {
                    state.hover = hover;
                    state.due = None;
                    shell.request_redraw();
                }
                if let Event::Window(window::Event::RedrawRequested(now)) = event {
                    if !hover && b.intersects(viewport) {
                        let due = state.due.get_or_insert(*now + delay);
                        if *now >= *due {
                            shell.publish(super::Event::Advance);
                            *due = *now + delay;
                        }
                        shell.request_redraw_at(*due);
                    } else {
                        state.due = None;
                    }
                }
            }
        }
        if let Some((l, t)) = layout.iter_mut(&mut tree.children).next() {
            self.child.update(t, event, l, cursor, r, shell, viewport);
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        r: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let Some((l, t)) = layout.iter(&tree.children).next() {
            self.child.draw(t, r, theme, style, l, cursor, viewport);
        }
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        v: &Rectangle,
        r: &Renderer,
    ) -> mouse::Interaction {
        layout
            .iter(&tree.children)
            .next()
            .map(|(l, t)| self.child.mouse_interaction(t, l, cursor, v, r))
            .unwrap_or_default()
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        r: &Renderer,
        v: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, super::Event<Message>, Theme, Renderer>> {
        overlay::from_children(
            std::slice::from_mut(&mut self.child),
            tree,
            layout,
            r,
            v,
            translation,
            window,
        )
    }
}
