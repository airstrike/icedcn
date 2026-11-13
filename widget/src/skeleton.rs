//! Skeleton loading placeholder.
//!
//! Mirrors shadcn's `<Skeleton>` primitive: a rounded muted block whose
//! dimensions are supplied by the call site. The fill pulses on a two-second cycle while the placeholder is visible.

use std::f32::consts::PI;
use std::marker::PhantomData;

use iced_core::Element;
use iced_core::Length::{self, Shrink};
use iced_core::time::Instant;
use iced_core::widget::{Meta, Tree, tree};
use iced_core::{
    Event, Layout, Rectangle, Renderer as _, Shell, Size, Widget, layout, mouse, renderer, window,
};
use iced_widget::Renderer;
use icedxdy_core::{Radius, Theme, radius};

use crate::compose::{Compose, Slot};

/// A muted placeholder block.
pub struct Skeleton<'a, Message> {
    width: Length,
    height: Length,
    radius: Radius,
    _marker: PhantomData<&'a Message>,
}

/// Create a [`Skeleton`] with `rounded-md` corners and no size; set one
/// with [`width`](Skeleton::width) and [`height`](Skeleton::height).
pub fn skeleton<'a, Message: 'a>() -> Skeleton<'a, Message> {
    Skeleton {
        width: Shrink,
        height: Shrink,
        radius: radius::MD.into(),
        _marker: PhantomData,
    }
}

impl<'a, Message: 'a> Skeleton<'a, Message> {
    /// Set the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the corner radius (`u32::MAX as f32` for `rounded-full`).
    pub fn radius(mut self, radius: impl Into<Radius>) -> Self {
        self.radius = radius.into();
        self
    }
}

impl<'a, Message: 'a> From<Skeleton<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(skeleton: Skeleton<'a, Message>) -> Self {
        Box::new(skeleton).compose()
    }
}

impl<'a, Message: 'a> From<Skeleton<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Skeleton<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Skeleton<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self)._boxed()
    }
}

struct Pulse {
    start: Instant,
    alpha: f32,
}
impl<Message> Meta for Skeleton<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Skeleton<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Pulse>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Pulse {
            start: Instant::now(),
            alpha: 1.0,
        })
    }
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(self.width, self.height, Size::ZERO);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        _: mouse::Cursor,
        _: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event
            && layout.bounds().intersects(viewport)
        {
            let pulse = tree.state.downcast_mut::<Pulse>();
            let phase = now.saturating_duration_since(pulse.start).as_secs_f32() * PI;
            pulse.alpha = 0.75 + 0.25 * phase.cos();
            shell.request_redraw();
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _: &renderer::Style,
        layout: Layout,
        _: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if !layout.bounds().intersects(viewport) {
            return;
        }
        let alpha = tree.state.downcast_ref::<Pulse>().alpha;
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: iced_core::Border {
                    radius: self.radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            theme.muted.base.scale_alpha(alpha),
        );
    }
}
