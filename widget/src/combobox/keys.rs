use crate::operation::focus;
use iced_core::keyboard::{self, Key};
use iced_core::widget::{Meta, Operation, tree::Tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

type Handler<'a, Message> = Box<dyn Fn(Key<&str>) -> Option<Message> + 'a>;

/// Listen to keys only while the wrapped control contains focus.
pub(crate) struct Keys<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub on_key: Handler<'a, Message>,
}
impl<Message> Meta for Keys<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Keys<'_, Message> {
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
