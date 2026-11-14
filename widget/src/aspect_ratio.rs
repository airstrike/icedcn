//! A layout-only wrapper that constrains its single child to a fixed
//! width-to-height ratio.
//!
//! The widget fills the available width by default and computes the height as `width / ratio`.
//! Apply visual styles directly to its child.
//!
//! ```ignore
//! use icedcn::widget::aspect_ratio;
//!
//! aspect_ratio(my_image, 16.0 / 9.0).max_width(384.0)
//! ```
//!
//! Use [`AspectRatio::max_width`] to cap the available width, or
//! [`AspectRatio::width`] for an explicit iced length. When the available
//! height is smaller, both axes shrink together to preserve the ratio.

use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{Element, Event, Layout, Length, Pixels, Rectangle, Shell, Size, Vector, Widget};
use iced_widget::Renderer;

use icedcn_core::Theme;

use crate::compose::{Compose, Inherited, Slot};

/// Constrains a child to the given width divided by height. For example, `16.0 / 9.0` produces
/// a widescreen frame. Use [`AspectRatio::width`] or [`AspectRatio::max_width`] to control its
/// width.
pub struct AspectRatio<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
{
    // A slot can receive inherited props until iced diffs the widget.
    pending: Option<Slot<'a, Message, Theme, Renderer>>,
    content: Option<Element<'a, Message, Theme, Renderer>>,
    ratio: f32,
    width: Length,
    max_width: f32,
}

impl<'a, Message: 'a, Renderer> AspectRatio<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer + 'a,
{
    /// Wrap `content` and constrain it to the given `width / height` ratio.
    pub fn new(content: impl Into<Slot<'a, Message, Theme, Renderer>>, ratio: f32) -> Self {
        Self {
            pending: Some(content.into()),
            content: None,
            ratio,
            width: Length::Fill,
            max_width: f32::INFINITY,
        }
    }

    /// Set the width policy for this aspect_ratio box. Defaults to
    /// `Length::Fill`. Use `Length::Fixed(n)` for an explicit pixel width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Cap the resolved width in logical pixels.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }
}

/// Constrain the child to a width-to-height ratio, such as `16.0 / 9.0`.
pub fn aspect_ratio<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
    ratio: f32,
) -> AspectRatio<'a, Message> {
    AspectRatio::new(content, ratio)
}

impl<Message, Renderer> Meta for AspectRatio<'_, Message, Renderer> where
    Renderer: iced_core::Renderer
{
}

impl<'a, Message: 'a, Renderer> Widget<Message, Theme, Renderer>
    for AspectRatio<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer + 'a,
{
    fn diff(&mut self, tree: &mut Tree) {
        if let Some(slot) = self.pending.take() {
            self.content = Some(slot.compose());
        }
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let ratio = if self.ratio.is_finite() && self.ratio > 0.0 {
            self.ratio
        } else {
            1.0
        };
        // A ratio projects the available width into a height even inside a
        // shrink-width parent. A completely unbounded width has no projection.
        let bounds = limits.width(self.width).bounds();
        let width = bounds.width.min(self.max_width).max(0.0);
        let width = if width.is_finite() { width } else { 0.0 };
        let child_size = Size::new(width, bounds.height.max(0.0)).ratio(ratio);

        let child_limits = layout::Limits::new(child_size, child_size);
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, &child_limits);
        child.translation = Vector::ZERO;

        tree.size = child_size;
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
        let (layout, tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("aspect ratio has a child");

        self.content
            .update(tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, tree) = layout
            .iter(&tree.children)
            .next()
            .expect("aspect ratio has a child");

        self.content
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (layout, tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("aspect ratio has a child");

        self.content
            .operate(tree, layout, viewport, renderer, operation);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let (layout, tree) = layout
            .iter(&tree.children)
            .next()
            .expect("aspect ratio has a child");

        self.content.draw(
            tree,
            renderer,
            theme,
            renderer_style,
            layout,
            cursor,
            viewport,
        );
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
        let (layout, tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("aspect ratio has a child");

        self.content
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        widget._boxed()
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>> for Slot<'a, Message> {
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a, Renderer: iced_core::Renderer + 'a> Compose<Message, Theme, Renderer>
    for AspectRatio<'a, Message, Renderer>
{
    fn inherit(&mut self, inherited: &Inherited) {
        if let Some(child) = &mut self.pending {
            child.inherit(inherited);
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self)._boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::aspect_ratio;
    use crate::{Inherited, Slot, button, container};
    use iced_core::Length::Fill;
    use iced_core::widget::Id;
    use iced_core::{Element, Length, Point, Size};
    use iced_test::Simulator;
    use iced_test::simulator::click;
    use iced_widget::Space;
    use icedcn_core::Theme;

    #[test]
    fn an_oversized_or_bounded_width_respects_its_available_space() {
        for (width, expected) in [(384.into(), 200.0), (Length::Fill.max(80), 80.0)] {
            let id = Id::unique();
            let child = container(Space::new())
                .id(id.clone())
                .width(Fill)
                .height(Fill);
            let content: Element<'_, (), Theme, iced_widget::Renderer> =
                aspect_ratio(child, 2.0).width(width).into();
            let mut ui = Simulator::with_size(Default::default(), Size::new(200.0, 300.0), content);
            let bounds = ui.find(id).unwrap().bounds();
            assert_eq!(bounds.size(), Size::new(expected, expected / 2.0));
        }
    }

    #[test]
    fn disabled_reaches_interactive_children() {
        let mut content: Slot<'_, bool> = aspect_ratio(
            button(Space::new().width(40).height(40)).on_press(true),
            1.0,
        )
        .width(64)
        .into();
        content.inherit(&Inherited::default().with_disabled(true));
        let mut ui = Simulator::new(content.compose());
        ui.point_at(Point::new(10.0, 10.0));
        ui.simulate(click());
        assert_eq!(ui.drain().count(), 0);
    }
}
