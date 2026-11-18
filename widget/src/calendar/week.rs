//! One week row: day cells side by side, painted in Nova's stacking order.
use super::Event;
use iced_core::Renderer as _;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Border, Element, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse,
    overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::calendar::Day;

pub(super) struct Week<'a> {
    /// One entry per child: the day it shows, if any.
    pub days: Vec<Option<Day>>,
    pub children: Vec<Element<'a, Event, Theme, Renderer>>,
    pub size: f32,
}

impl Meta for Week<'_> {}
impl Widget<Event, Theme, Renderer> for Week<'_> {
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }
    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.size * self.children.len() as f32),
            Length::Fixed(self.size),
        )
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, _: &layout::Limits) {
        let limits = layout::Limits::new(Size::ZERO, Size::new(self.size, self.size));
        for (i, (cell, child)) in self.children.iter_mut().zip(&mut tree.children).enumerate() {
            cell.layout(child, renderer, &limits);
            child.translation = Vector::new(
                i as f32 * self.size + (self.size - child.size.width) / 2.0,
                (self.size - child.size.height) / 2.0,
            );
        }
        tree.size = Size::new(self.size * self.children.len() as f32, self.size);
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
        let cells = || {
            self.days
                .iter()
                .zip(&self.children)
                .zip(layout.iter(&tree.children))
        };
        // The cells, and the range ends isolated inside them (`isolate z-0`),
        // paint first; every other day button (`z-10`) paints above them.
        for ((day, cell), (layout, tree)) in cells() {
            let Some(day) = day else { continue };
            if let Some((color, radius)) = day.cell(theme) {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle::new(layout.position(), Size::new(self.size, self.size)),
                        border: Border {
                            radius: radius.into(),
                            ..Border::default()
                        },
                        snap: true,
                        ..renderer::Quad::default()
                    },
                    color,
                );
            }
            if day.layered() {
                cell.draw(tree, renderer, theme, style, layout, cursor, viewport);
            }
        }
        for ((day, cell), (layout, tree)) in cells() {
            if !day.is_some_and(|day| day.layered()) {
                cell.draw(tree, renderer, theme, style, layout, cursor, viewport);
            }
        }
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            for (cell, (layout, tree)) in self
                .children
                .iter_mut()
                .zip(layout.iter_mut(&mut tree.children))
            {
                cell.operate(tree, layout, viewport, renderer, operation);
            }
        });
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
        for (cell, (layout, tree)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            cell.update(tree, event, layout, cursor, renderer, shell, viewport);
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
        self.children
            .iter()
            .zip(layout.iter(&tree.children))
            .map(|(cell, (layout, tree))| {
                cell.mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
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
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
