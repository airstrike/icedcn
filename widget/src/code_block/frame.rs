//! Paint the content's keyboard ring around the complete panel.
use iced_core::Widget;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Border, Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedxdy_core::{Theme, radius};

pub(super) struct Frame<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    actions: bool,
}
impl<'a, Message> Frame<'a, Message> {
    pub(super) fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            content,
            actions: false,
        }
    }
    pub(super) fn actions(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            content,
            actions: true,
        }
    }
}
impl<Message> Meta for Frame<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Frame<'_, Message> {
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
        if self.actions {
            tree.size.width = (tree.size.width - 6.).max(0.);
            tree.size.height = (tree.size.height - 8.).max(0.);
            tree.children[0].translation.y = -4.;
        }
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        op: &mut dyn Operation,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
        self.content.operate(child, layout, viewport, renderer, op);
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
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
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
        if !self.actions && super::viewport::focused(tree) {
            let b = layout.bounds();
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: b.x - 3.,
                        y: b.y - 3.,
                        width: b.width + 6.,
                        height: b.height + 6.,
                    },
                    border: Border {
                        width: 3.,
                        radius: (radius::LG + 3.).into(),
                        color: theme.neutral.ring.scale_alpha(0.5).into(),
                    },
                    ..Default::default()
                },
                iced_core::Color::TRANSPARENT,
            );
        }
        let (layout, child) = layout.iter(&tree.children).next().expect("frame child");
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
        let (layout, child) = layout.iter(&tree.children).next().expect("frame child");
        self.content
            .mouse_interaction(child, layout, cursor, viewport, renderer)
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
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
        self.content
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}
