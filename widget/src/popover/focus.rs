//! Return keyboard focus to a popover's trigger when `Escape` closes it.
//!
//! [`Escape`] wraps the open panel and reports an `Escape` that nothing
//! inside it claimed, which the anchored overlay is about to take as a
//! dismissal. [`Refocus`] always wraps the trigger, so its state survives,
//! and focuses it while a refocus is pending, with its ring showing, as a
//! keyboard user expects. A pointer dismissal never asks for it.

use crate::operation::focus;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::{Meta, Operation, tree::Tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

/// Publishes `on_escape` for an `Escape` the panel's content left unclaimed.
pub(super) struct Escape<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub on_escape: fn() -> Message,
}

/// While `pending`, focuses the first control inside it and publishes
/// `on_focus`, which clears the request.
pub(super) struct Refocus<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub pending: bool,
    pub on_focus: fn() -> Message,
}

impl<Message> Meta for Escape<'_, Message> {}
impl<Message> Meta for Refocus<'_, Message> {}

macro_rules! delegate {
    () => {
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
                .expect("a focus wrapper has one child");
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
                .expect("a focus wrapper has one child");
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
                .expect("a focus wrapper has one child");
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
                .expect("a focus wrapper has one child");
            self.content.overlay(
                child_tree,
                child_layout,
                renderer,
                viewport,
                translation,
                window,
            )
        }
    };
}

impl<Message> Widget<Message, Theme, Renderer> for Escape<'_, Message> {
    delegate!();

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
            .expect("a focus wrapper has one child");
        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
        // Left uncaptured, so the anchored overlay still dismisses on it.
        if !shell.is_event_captured()
            && let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event
            && key.as_ref() == Key::Named(Named::Escape)
        {
            shell.publish((self.on_escape)());
        }
    }
}

impl<Message> Widget<Message, Theme, Renderer> for Refocus<'_, Message> {
    delegate!();

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
            .expect("a focus wrapper has one child");
        if self.pending {
            self.pending = false;
            self.content.operate(
                child_tree,
                child_layout,
                viewport,
                renderer,
                &mut focus::select(Some(0)),
            );
            shell.publish((self.on_focus)());
            shell.request_redraw();
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
}
