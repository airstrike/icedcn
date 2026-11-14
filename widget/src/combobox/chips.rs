//! A wrapping row of chips with an editable input that fills its last line.
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

use crate::Slot;

pub(super) struct Chips<'a, Message> {
    children: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message: 'a> Chips<'a, Message> {
    pub fn new(children: Vec<Slot<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().map(Slot::compose).collect(),
        }
    }
}

impl<Message> Meta for Chips<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Chips<'_, Message> {
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        const GAP: f32 = 4.;
        const INPUT_MIN: f32 = 64.;
        let input = self.children.len() - 1;
        let inset = if input == 0 { 11. } else { 5. };
        let width = limits.bounds().width;
        let inner = (width - inset * 2.).max(0.);
        let mut x = 0.;
        let mut y = 4.;
        let mut line = 0_f32;
        for (i, (child, node)) in self.children.iter_mut().zip(&mut tree.children).enumerate() {
            let min = if i == input {
                INPUT_MIN.min(inner)
            } else {
                child.layout(node, renderer, &limits.width(inner).loose());
                node.size.width
            };
            if x > 0. && x + min > inner {
                y += line + GAP;
                line = 0.;
                x = 0.;
            }
            if i == input {
                child.layout(node, renderer, &limits.loose().width((inner - x).max(0.)));
            }
            node.translation = Vector::new(inset + x, y + (24. - node.size.height).max(0.) / 2.);
            x += node.size.width + GAP;
            line = line.max(24.).max(node.size.height);
        }
        tree.size = Size::new(width, y + line + 4.);
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
        operation.traverse(&mut |op| {
            for (child, (layout, state)) in self
                .children
                .iter_mut()
                .zip(layout.iter_mut(&mut tree.children))
            {
                child.operate(state, layout, viewport, renderer, op);
            }
        });
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
        for (child, (layout, state)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            child.update(state, event, layout, cursor, renderer, shell, viewport);
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
        for (child, (layout, state)) in self.children.iter().zip(layout.iter(&tree.children)) {
            child.draw(state, renderer, theme, style, layout, cursor, viewport);
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
            .map(|(child, (layout, state))| {
                child.mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }
}
