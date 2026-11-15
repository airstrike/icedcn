use super::Orientation;
use crate::operation::{
    control::{self, Control},
    focus,
    scroll::Reveal,
};
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::{Id, Meta, Operation, tree::Tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;
use std::any::Any;

// List-level keyboard navigation operates on the actual mounted controls, so
// it's independent of construction order and naturally skips disabled triggers.
pub(crate) struct List<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub orientation: Option<(Orientation, Option<Id>)>,
    pub single_tab_stop: bool,
}

struct Activate {
    index: usize,
    current: usize,
}
impl Operation for Activate {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if let Some(control) = state.downcast_mut::<Control>() {
            if self.current == self.index {
                control::activate().custom(id, bounds, control);
            }
            self.current += 1;
        }
    }
}
impl<Message> Meta for List<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for List<'_, Message> {
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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a roving group has one child");

        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(child_tree, child_layout, viewport, renderer, op)
        };
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            ..
        }) = event
        {
            let count = focus::count(&mut visit);
            if count.focused.is_some() {
                let target = match (
                    key,
                    self.orientation
                        .as_ref()
                        .map(|(orientation, _)| *orientation),
                ) {
                    (Named::Home, _) => Some(0),
                    (Named::End, _) => Some(count.total - 1),
                    (Named::ArrowRight, Some(Orientation::Horizontal) | None)
                    | (Named::ArrowDown, Some(Orientation::Vertical) | None) => {
                        focus::adjacent(count, false)
                    }
                    (Named::ArrowLeft, Some(Orientation::Horizontal) | None)
                    | (Named::ArrowUp, Some(Orientation::Vertical) | None) => {
                        focus::adjacent(count, true)
                    }
                    _ => None,
                };
                if let Some(index) = target {
                    if let Some((_, Some(scroll))) = &self.orientation {
                        focus::reveal(index, scroll, &mut visit);
                    } else {
                        visit(&mut focus::select(Some(index)));
                    }
                    visit(&mut Activate { index, current: 0 });
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
        }
        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
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
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a roving group has one child");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a roving group has one child");

        self.content
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a roving group has one child");

        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(child_tree, child_layout, viewport, renderer, op)
        };
        if self.single_tab_stop {
            let mut active = Active::default();
            visit(&mut active);
            let index = focus::count(&mut visit)
                .focused
                .or(active.selected)
                .unwrap_or(0);
            let entered = focus::roving(operation, |i| i == index, &mut visit);
            if let (Some((_, Some(scroll))), Some(target)) = (&self.orientation, entered) {
                visit(&mut Reveal {
                    scroll: scroll.clone(),
                    target,
                });
            }
        } else {
            visit(operation);
        }
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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a roving group has one child");

        self.content.overlay(
            child_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}

#[derive(Default)]
struct Active {
    count: usize,
    selected: Option<usize>,
}
impl Operation for Active {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if let Some(control) = state.downcast_mut::<Control>() {
            let mut checked = control::checked();
            checked.custom(id, bounds, control);
            if checked.get() == Some(true) {
                self.selected = Some(self.count);
            }
            self.count += 1;
        }
    }
}
