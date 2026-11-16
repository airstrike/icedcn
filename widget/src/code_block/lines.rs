//! Extend line tints over the full code width without compressing long lines,
//! and keep the line numbers in place while the code scrolls sideways.
use super::{LineStyle, selection};
use iced_core::Widget;
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Meta, Operation};
use iced_core::{
    Element, Event, Layout, Length, Point, Rectangle, Renderer as _, Shell, Size, Vector, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

/// Line numbers that stay at the left while the code scrolls sideways. The
/// code block's viewport sets `offset` to its horizontal scroll offset; the
/// numbers draw that much further right, and the code is clipped at `width`.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Gutter {
    /// The horizontal scroll offset the numbers undo.
    pub(super) offset: f32,
    /// Where the code starts, from the left edge; zero without line numbers.
    pub(super) width: f32,
}

impl Gutter {
    /// The pinned strip, in the content's coordinates.
    pub(super) fn strip(self, bounds: Rectangle) -> Rectangle {
        Rectangle {
            x: bounds.x + self.offset,
            width: self.width,
            ..bounds
        }
    }

    /// The part of `viewport` the code may draw in.
    pub(super) fn code(self, bounds: Rectangle, viewport: Rectangle) -> Rectangle {
        let left = (bounds.x + self.offset + self.width).max(viewport.x);
        Rectangle {
            x: left,
            width: (viewport.x + viewport.width - left).max(0.),
            ..viewport
        }
    }

    /// Whether a press at `cursor` lands on the pinned numbers.
    pub(super) fn is_pressed(
        self,
        event: &Event,
        cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> bool {
        self.width > 0.
            && matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)))
            && cursor.is_over(self.strip(bounds))
    }
}

pub(super) struct Lines<'a, Message> {
    /// The code column, then one gutter cell per row.
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    styles: Vec<LineStyle>,
    pin: f32,
}
impl<'a, Message> Lines<'a, Message> {
    pub(super) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        styles: Vec<LineStyle>,
        gutter: Vec<Element<'a, Message, Theme, Renderer>>,
        pin: f32,
    ) -> Self {
        let mut children = vec![content];
        children.extend(gutter);
        Self {
            children,
            styles,
            pin,
        }
    }
}
impl<Message> Meta for Lines<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Lines<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Gutter>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Gutter {
            offset: 0.,
            width: self.pin,
        })
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.state.downcast_mut::<Gutter>().width = self.pin;
        tree.diff_children(&mut self.children);
    }
    fn size(&self) -> Size<Length> {
        self.children[0].size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let (content, cells) = self.children.split_first_mut().expect("line column");
        let (column, trees) = tree.children.split_first_mut().expect("line column");
        content.layout(column, renderer, limits);
        tree.size = column.size;
        tree.size.width = tree.size.width.max(limits.max.width);
        for ((cell, cell_tree), row) in cells.iter_mut().zip(trees).zip(&column.children) {
            let cell_limits = layout::Limits::new(Size::ZERO, Size::new(self.pin, row.size.height));
            cell.layout(cell_tree, renderer, &cell_limits);
            cell_tree.translation = Vector::new(0., column.translation.y + row.translation.y);
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
        let gutter = tree.state.downcast_mut::<Gutter>();
        op.custom(None, layout.bounds(), gutter);
        let offset = Vector::new(gutter.offset, 0.);
        let mut children = layout.iter_mut(&mut tree.children);
        let (column_layout, column) = children.next().expect("line column");
        let rows = column_layout
            .iter(&column.children)
            .map(|(row, _)| row.bounds())
            .collect();
        let content = &mut self.children[0];
        let mut selected = selection::Selected::new(rows);
        content.operate(column, column_layout, viewport, renderer, &mut selected);
        let separators = selected.separators();
        let mut index = 0;
        content.operate(
            column,
            column_layout,
            viewport,
            renderer,
            &mut selection::Lines {
                inner: op,
                separators: &separators,
                index: &mut index,
            },
        );
        for ((cell_layout, cell_tree), cell) in children.zip(&mut self.children[1..]) {
            let moved = cell_layout.move_to(cell_layout.position() + offset);
            cell.operate(cell_tree, moved, viewport, renderer, op);
        }
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
        let gutter = *tree.state.downcast_ref::<Gutter>();
        // The code under the numbers is hidden: a press there selects nothing.
        let cursor = if gutter.is_pressed(event, cursor, layout.bounds()) {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("line column");
        self.children[0].update(child, event, layout, cursor, renderer, shell, viewport);
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
        let bounds = layout.bounds();
        let gutter = *tree.state.downcast_ref::<Gutter>();
        let mut children = layout.iter(&tree.children);
        let (lines_layout, lines_tree) = children.next().expect("line column");
        for ((row_layout, _), variant) in lines_layout.iter(&lines_tree.children).zip(&self.styles)
        {
            if let Some(background) = super::line_style(theme, *variant).background {
                let row = Rectangle {
                    x: bounds.x,
                    width: bounds.width,
                    ..row_layout.bounds()
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: row,
                        ..Default::default()
                    },
                    background,
                );
            }
        }
        if gutter.width == 0. {
            self.children[0].draw(
                lines_tree,
                renderer,
                theme,
                style,
                lines_layout,
                cursor,
                viewport,
            );
            return;
        }
        let code = gutter.code(bounds, *viewport);
        renderer.with_layer(code, |renderer| {
            self.children[0].draw(
                lines_tree,
                renderer,
                theme,
                style,
                lines_layout,
                cursor,
                &code,
            );
        });
        let offset = Vector::new(gutter.offset, 0.);
        for ((cell_layout, cell_tree), cell) in children.zip(&self.children[1..]) {
            let moved = cell_layout.move_to(cell_layout.position() + offset);
            if moved.bounds().intersection(viewport).is_some() {
                cell.draw(cell_tree, renderer, theme, style, moved, cursor, viewport);
            }
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
        let gutter = *tree.state.downcast_ref::<Gutter>();
        if gutter.width > 0. && cursor.is_over(gutter.strip(layout.bounds())) {
            return mouse::Interaction::default();
        }
        let (layout, child) = layout.iter(&tree.children).next().expect("line column");
        self.children[0].mouse_interaction(child, layout, cursor, viewport, renderer)
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
        self.children[0].overlay(child, layout, renderer, viewport, translation, window)
    }
}

/// A point on the line at `y`, at the start of the code.
pub(super) fn line_start(bounds: Rectangle, gutter: Gutter, y: f32) -> Point {
    Point::new(bounds.x + gutter.width, y)
}
