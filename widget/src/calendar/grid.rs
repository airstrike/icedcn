//! Calendar-specific roving focus. Buttons still own press, hover and focus paint.
use super::{Date, Event};
use crate::operation::focus;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::operation::{Focusable, Scrollable, Text, TextInput};
use iced_core::widget::{Id, Meta, Operation, Tree, tree};
use iced_core::{
    Element, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer, touch,
};
use iced_widget::Renderer;
use icedcn_core::Theme;
use std::any::Any;

pub(super) struct Grid<'a> {
    pub content: Element<'a, Event, Theme, Renderer>,
    // None is a header control; Some is an enabled day, in tree traversal order.
    pub targets: Vec<Option<Date>>,
    pub preferred: Option<usize>,
    pub in_month: Vec<usize>,
    pub focused: Option<Date>,
    pub generation: u64,
}
#[derive(Default)]
struct State {
    applied: Option<(Date, u64)>,
}
impl Meta for Grid<'_> {}
impl Widget<Event, Theme, Renderer> for Grid<'_> {
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
        let state = tree.state.downcast_mut::<State>();
        let requested = self.focused.map(|day| (day, self.generation));
        if requested != state.applied {
            if let Some(target) = self.focused.and_then(|date| {
                self.in_month
                    .iter()
                    .copied()
                    .find(|&i| self.targets[i] == Some(date))
                    .or_else(|| self.targets.iter().position(|d| *d == Some(date)))
            }) {
                let child = &mut tree.children[0];
                let layout = Layout::new(child.size);
                self.content.operate(
                    child,
                    layout,
                    &layout.bounds(),
                    renderer,
                    &mut focus::select(Some(target)),
                );
                state.applied = requested;
            } else if self.focused.is_none() {
                state.applied = None;
            }
        }
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced_core::Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Event>,
        viewport: &Rectangle,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("calendar content");
        if let iced_core::Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            modifiers,
            ..
        }) = event
            && matches!(
                key,
                Named::ArrowLeft
                    | Named::ArrowRight
                    | Named::ArrowUp
                    | Named::ArrowDown
                    | Named::Home
                    | Named::End
                    | Named::PageUp
                    | Named::PageDown
            )
        {
            let count =
                focus::count(|op| self.content.operate(child, layout, viewport, renderer, op));
            if let Some(day) = count
                .focused
                .and_then(|i| self.targets.get(i))
                .copied()
                .flatten()
            {
                shell.publish(Event::Navigate(day, *key, modifiers.shift()));
                shell.capture_event();
                return;
            }
        }
        // A pressed button captures the event before later siblings can blur.
        // Clear this grid's old focus first so only the clicked day receives
        // subsequent arrow keys, even when it precedes the old day in the tree.
        if !shell.is_event_captured()
            && matches!(
                event,
                iced_core::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | iced_core::Event::Touch(touch::Event::FingerPressed { .. })
            )
        {
            self.content.operate(
                child,
                layout,
                viewport,
                renderer,
                &mut focus::restore(None, false),
            );
        }
        self.content
            .update(child, event, layout, cursor, renderer, shell, viewport);
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
        let (layout, child) = layout
            .iter(&tree.children)
            .next()
            .expect("calendar content");
        self.content
            .draw(child, renderer, theme, style, layout, cursor, viewport);
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, child) = layout
            .iter(&tree.children)
            .next()
            .expect("calendar content");
        self.content
            .mouse_interaction(child, layout, cursor, viewport, renderer)
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("calendar content");
        let count = focus::count(|op| self.content.operate(child, layout, viewport, renderer, op));
        let target = count
            .focused
            .filter(|&i| self.targets.get(i).is_some_and(Option::is_some))
            .or(self.preferred);
        self.content.operate(
            child,
            layout,
            viewport,
            renderer,
            &mut Roving {
                operation,
                targets: &self.targets,
                target,
                index: 0,
                day: false,
            },
        );
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Event, Theme, Renderer>> {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("calendar content");
        self.content
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}

// Hide other dates from Tab traversal while keeping text, selectors, scrolling,
// and header controls visible to the original operation.
struct Roving<'a> {
    operation: &'a mut dyn Operation,
    targets: &'a [Option<Date>],
    target: Option<usize>,
    index: usize,
    day: bool,
}
impl Operation for Roving<'_> {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        let mut index = self.index;
        let mut day = self.day;
        self.operation.traverse(&mut |operation| {
            let mut nested = Roving {
                operation,
                targets: self.targets,
                target: self.target,
                index,
                day,
            };
            visit(&mut nested);
            index = nested.index;
            day = nested.day;
        });
        self.index = index;
        self.day = day;
    }
    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        self.day = self.targets.get(self.index).is_some_and(Option::is_some)
            && Some(self.index) != self.target;
        if !self.day {
            self.operation.focusable(id, bounds, state);
        }
        self.index += 1;
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
        self.operation.container(id, bounds, viewport);
    }
    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Text) {
        self.operation.text(id, bounds, state);
    }
    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if !self.day || !state.is::<focus::Visible>() {
            self.operation.custom(id, bounds, state);
        }
    }
    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        self.operation
            .scrollable(id, bounds, content, translation, state);
    }
    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
        self.operation.text_input(id, bounds, state);
    }
}
