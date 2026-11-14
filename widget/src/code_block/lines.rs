//! Extend line tints over the full code width without compressing long lines.
use super::{LineStyle, selection};
use iced_core::Widget;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, layout, mouse,
    overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

pub(super) struct Lines<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    styles: Vec<LineStyle>,
}
impl<'a, Message> Lines<'a, Message> {
    pub(super) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        styles: Vec<LineStyle>,
    ) -> Self {
        Self { content, styles }
    }
}
impl<Message> Meta for Lines<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Lines<'_, Message> {
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
        tree.size.width = tree.size.width.max(limits.max.width);
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
            .expect("line column");
        let rows = layout
            .iter(&child.children)
            .map(|(row, _)| row.bounds())
            .collect();
        let mut selected = selection::Selected::new(rows);
        self.content
            .operate(child, layout, viewport, renderer, &mut selected);
        let separators = selected.separators();
        let mut index = 0;
        self.content.operate(
            child,
            layout,
            viewport,
            renderer,
            &mut selection::Lines {
                inner: op,
                separators: &separators,
                index: &mut index,
            },
        );
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
            .expect("line column");
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
        let (lines_layout, lines_tree) = layout.iter(&tree.children).next().expect("line column");
        for ((row_layout, _), variant) in lines_layout.iter(&lines_tree.children).zip(&self.styles)
        {
            if let Some(background) = super::line_style(theme, *variant).background {
                let bounds = Rectangle {
                    x: layout.bounds().x,
                    width: layout.bounds().width,
                    ..row_layout.bounds()
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        ..Default::default()
                    },
                    background,
                );
            }
        }
        let (layout, child) = layout.iter(&tree.children).next().expect("line column");
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
        let (layout, child) = layout.iter(&tree.children).next().expect("line column");
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
            .expect("line column");
        self.content
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}
