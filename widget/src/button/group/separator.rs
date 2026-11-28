//! An independently oriented, inset divider inside a button group.
use crate::compose::{Compose, View};
use iced_core::widget::{Meta, Tree};
use iced_core::{
    Background, Element, Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer,
};
use iced_widget::{Renderer, Widget as _};
use icedcn_core::{Orientation, Theme};

/// A one-pixel divider. Its orientation is vertical unless explicitly changed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Separator {
    orientation: Orientation,
}
impl Separator {
    pub(super) fn new() -> Self {
        Self {
            orientation: Orientation::Vertical,
        }
    }
    /// Set the separator's own axis, independently of the containing group.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }
}
impl Meta for Separator {}
impl<Message> Widget<Message, Theme, Renderer> for Separator {
    fn size(&self) -> Size<Length> {
        match self.orientation {
            Orientation::Vertical => Size::new(1.into(), Length::Fill),
            Orientation::Horizontal => Size::new(Length::Fill, 1.into()),
        }
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        let size = <Self as Widget<Message, Theme, Renderer>>::size(self);
        tree.size = layout::atomic(limits, size.width, size.height);
    }
    fn draw(
        &self,
        _: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _: &renderer::Style,
        layout: Layout,
        _: mouse::Cursor,
        _: &Rectangle,
    ) {
        use iced_core::Renderer as _;
        let mut bounds = layout.bounds();
        match self.orientation {
            Orientation::Vertical => {
                bounds.y += 1.;
                bounds.height = (bounds.height - 2.).max(0.);
            }
            Orientation::Horizontal => {
                bounds.x += 1.;
                bounds.width = (bounds.width - 2.).max(0.);
            }
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            Background::Color(theme.input.into()),
        );
    }
}
impl<Message> Compose<Message> for Separator {
    fn compose<'a>(self: Box<Self>) -> Element<'a, Message, Theme, Renderer>
    where
        Self: 'a,
    {
        (*self).boxed()
    }
}
impl<'a, Message: 'a> From<Separator> for View<'a, Message> {
    fn from(value: Separator) -> Self {
        View::new(value)
    }
}
impl<'a, Message: 'a> From<Separator> for Element<'a, Message, Theme, Renderer> {
    fn from(value: Separator) -> Self {
        value.boxed()
    }
}
