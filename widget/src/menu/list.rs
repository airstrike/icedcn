use crate::operation::focus;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::{
    Meta, Operation,
    tree::{self, Tree},
};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

/// The keyboard focus scope of a horizontal menubar or a vertical menu panel.
pub(crate) struct List<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    /// None manages initial focus only; the input owns its editing keys.
    pub horizontal: Option<bool>,
    pub autofocus: bool,
    pub on_move: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}
impl<Message> Meta for List<'_, Message> {}
#[derive(Default)]
struct State {
    initialized: bool,
}
impl<Message> Widget<Message, Theme, Renderer> for List<'_, Message> {
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

        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(child_tree, child_layout, viewport, renderer, op)
        };
        let state = tree.state.downcast_mut::<State>();
        if !state.initialized {
            state.initialized = true;
            if self.autofocus {
                visit(&mut focus::select(Some(0)));
                shell.request_redraw();
            }
        }
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            ..
        }) = event
        {
            let count = focus::count(&mut visit);
            if count.focused.is_some() {
                let next = match (key, self.horizontal) {
                    (Named::Home, Some(_)) => Some(0),
                    (Named::End, Some(_)) => Some(count.total - 1),
                    (Named::ArrowLeft, Some(true)) | (Named::ArrowUp, Some(false)) => {
                        focus::adjacent(count, true)
                    }
                    (Named::ArrowRight, Some(true)) | (Named::ArrowDown, Some(false)) => {
                        focus::adjacent(count, false)
                    }
                    _ => None,
                };
                if let Some(index) = next {
                    visit(&mut focus::select(Some(index)));
                    if let Some(moved) = &self.on_move {
                        shell.publish(moved(index));
                    }
                    shell.request_redraw();
                    shell.capture_event();
                    return;
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
