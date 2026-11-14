//! Flex layout and focus stacking for joined controls.
use crate::operation::focus;
use iced_core::layout::flex::{self, Axis};
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Alignment, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size, Vector, Widget,
    layout, mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::{Orientation, Theme};

pub(super) struct Group<'a, Message> {
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    orientation: Orientation,
    width: Length,
    spacing: f32,
}
#[derive(Default)]
struct State {
    flex: flex::Cache,
    focused: Option<usize>,
}
impl<'a, Message> Group<'a, Message> {
    pub(super) fn new(
        children: Vec<Element<'a, Message, Theme, Renderer>>,
        orientation: Orientation,
        width: Length,
        spacing: f32,
    ) -> Self {
        Self {
            children,
            orientation,
            width,
            spacing,
        }
    }
    fn focus(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) {
        let focused = self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
            .position(|(child, (layout, tree))| {
                focus::count(|op| child.operate(tree, layout, viewport, renderer, op))
                    .focused
                    .is_some()
            });
        tree.state.downcast_mut::<State>().focused = focused;
    }
}
impl<Message> Meta for Group<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Group<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let axis = match self.orientation {
            Orientation::Horizontal => Axis::Horizontal,
            Orientation::Vertical => Axis::Vertical,
        };
        tree.size = flex::resolve(
            axis,
            renderer,
            limits,
            self.width,
            Length::Shrink,
            Padding::ZERO,
            self.spacing,
            Alignment::Start,
            &mut tree.children,
            &mut self.children,
            &mut tree.state.downcast_mut::<State>().flex,
        );
        // CSS items-stretch: every surface reaches the measured cross-axis edge.
        // Retain the flex allocation on the main axis, including Fill inputs.
        for (child, state) in self.children.iter_mut().zip(&mut tree.children) {
            let size = match self.orientation {
                Orientation::Horizontal => Size::new(state.size.width, tree.size.height),
                Orientation::Vertical => Size::new(tree.size.width, state.size.height),
            };
            if size == state.size {
                continue;
            }
            let offset = state.translation;
            child.layout(state, renderer, &layout::Limits::new(size, size));
            state.translation = offset;
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
            for (child, (layout, tree)) in self
                .children
                .iter_mut()
                .zip(layout.iter_mut(&mut tree.children))
            {
                child.operate(tree, layout, viewport, renderer, operation);
            }
        });
        self.focus(tree, layout, viewport, renderer);
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
        for (child, (layout, tree)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            child.update(tree, event, layout, cursor, renderer, shell, viewport);
        }
        self.focus(tree, layout, viewport, renderer);
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
            .map(|(child, (layout, tree))| {
                child.mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
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
        let focused = tree.state.downcast_ref::<State>().focused;
        // The focused surface paints last, so its exterior ring stays visible
        // over adjacent controls. Neither the group nor its children are masked.
        for focused_pass in [false, true] {
            for (index, (child, (layout, tree))) in self
                .children
                .iter()
                .zip(layout.iter(&tree.children))
                .enumerate()
            {
                if (focused == Some(index)) == focused_pass {
                    child.draw(tree, renderer, theme, style, layout, cursor, viewport);
                }
            }
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
