//! Keyboard selection and active-row visibility for a command palette.
use crate::operation::{focus, scroll::Reveal};
use iced_core::keyboard::{self, Key};
use iced_core::shell::Bus;
use iced_core::widget::{
    Id, Meta, Operation,
    tree::{self, Tree},
};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

type Handler<'a, Message> = Box<dyn Fn(Key<&str>) -> Option<Message> + 'a>;

/// Listen to keys only while the wrapped control contains focus.
pub(super) struct Behavior<'a, Message> {
    pub reveal: Option<(Id, Id)>,
    pub content: Element<'a, Message, Theme, Renderer>,
    pub on_key: Handler<'a, Message>,
    pub is_hover: fn(&Message) -> bool,
}
impl<Message> Meta for Behavior<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Behavior<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Option<Id>>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(None::<Id>)
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
            .expect("a keyboard handler has one child");

        let revealed = tree.state.downcast_mut::<Option<Id>>();
        if let Some((scroll, target)) = &self.reveal {
            if revealed.as_ref() != Some(target) {
                let mut find = Find {
                    target: target.clone(),
                    bounds: None,
                };
                self.content
                    .operate(child_tree, child_layout, viewport, renderer, &mut find);
                if let Some(bounds) = find.bounds {
                    self.content.operate(
                        child_tree,
                        child_layout,
                        viewport,
                        renderer,
                        &mut Reveal {
                            scroll: scroll.clone(),
                            target: bounds,
                        },
                    );
                    *revealed = Some(target.clone());
                    shell.request_redraw();
                }
            }
        } else {
            *revealed = None;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event
            && (matches!(key, Key::Named(_))
                || !(modifiers.command() || modifiers.control() || modifiers.alt()))
            && let Some(message) = (self.on_key)(key.as_ref())
            && focus::count(|op| {
                self.content
                    .operate(child_tree, child_layout, viewport, renderer, op)
            })
            .focused
            .is_some()
        {
            shell.publish(message);
            shell.capture_event();
            return;
        }
        let mut bus = Bus::new();
        {
            let mut local = shell.local(&mut bus);
            self.content.update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                &mut local,
                viewport,
            );
            if local.is_event_captured() {
                shell.capture_event();
            }
            shell.request_redraw_at(local.redraw_request());
            shell.invalidate(local.invalidation());
            shell.request_input_method(local.input_method());
            shell.clipboard_mut().merge(local.clipboard_mut());
        }
        // MouseArea also reports local movement when scrolling changes its bounds.
        // Only a real pointer move may replace the keyboard's selected command.
        let moved = matches!(event, Event::Mouse(mouse::Event::CursorMoved { .. }));
        for (message, receipt) in bus.drain() {
            if moved || !(self.is_hover)(&message) {
                shell.forward(message, receipt);
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
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a keyboard handler has one child");

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
            .expect("a keyboard handler has one child");

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
            .expect("a keyboard handler has one child");

        self.content
            .operate(child_tree, child_layout, viewport, renderer, operation);
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
            .expect("a keyboard handler has one child");

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

struct Find {
    target: Id,
    bounds: Option<Rectangle>,
}
impl Operation for Find {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        if id == Some(&self.target) {
            self.bounds = Some(bounds);
        }
    }
}
