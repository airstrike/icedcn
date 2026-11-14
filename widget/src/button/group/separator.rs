//! [`Separator`] for use inside a [`super::ButtonGroup`].
//!
//! The one-pixel divider runs perpendicular to the group and uses the active theme’s
//! neutral border color.
//!
//! Constructed via [`super::separator()`]; the parent `ButtonGroup` wraps
//! each instance in `super::Item::Separator`.

use iced_core::Length;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::{Meta, Tree};
use iced_core::{Background, Layout, Rectangle, Size, Widget};

use icedcn_core::Theme;

use super::Orientation;

/// A 1px divider rendered between adjacent buttons in a [`super::ButtonGroup`].
///
/// The group sets the orientation during composition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Separator {
    /// The *group's* orientation. The separator draws perpendicular to it.
    pub(super) group_orientation: Orientation,
    /// Length of the divider in logical pixels, supplied by the group. A zero length
    /// falls back to a one-pixel square.
    pub(super) cross_axis: f32,
}

impl Separator {
    /// Internal constructor. The public surface is
    /// [`super::separator()`].
    pub(super) fn new() -> Self {
        Self {
            group_orientation: Orientation::Horizontal,
            cross_axis: 0.0,
        }
    }
}

impl Meta for Separator {}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Separator
where
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        // Perpendicular axis is sized to the cluster's natural max button
        // height/width (set by the parent group); the along-axis is fixed
        // at 1px. Zero cross_axis falls back to 1.0 so the separator stays
        // visible in degenerate "no buttons" cases.
        let cross = if self.cross_axis > 0.0 {
            self.cross_axis
        } else {
            1.0
        };
        match self.group_orientation {
            Orientation::Horizontal => Size {
                width: Length::Fixed(1.0),
                height: Length::Fixed(cross),
            },
            Orientation::Vertical => Size {
                width: Length::Fixed(cross),
                height: Length::Fixed(1.0),
            },
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        let size = <Self as Widget<Message, Theme, Renderer>>::size(self);
        tree.size = layout::atomic(limits, size.width, size.height);
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bg: iced_core::Color = theme.neutral.border.into();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            Background::Color(bg),
        );
    }
}
