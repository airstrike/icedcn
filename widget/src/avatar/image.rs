//! Select the fallback until the renderer has decoded the portrait.
use iced_core::Length::Fill;
use iced_core::image::{Handle, Renderer as _};
use iced_core::widget::{Meta, Tree};
use iced_core::{ContentFit, Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
use iced_widget::{Image, Renderer};
use icedxdy_core::Theme;

use crate::Slot;

pub(super) struct Portrait<'a, Message> {
    handle: Handle,
    image: Image,
    fallback: iced_core::Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message: 'a> Portrait<'a, Message> {
    pub(super) fn new(handle: Handle, fallback: Slot<'a, Message>) -> Self {
        let image = Image::new(handle.clone())
            .width(Fill)
            .height(Fill)
            .content_fit(ContentFit::Cover)
            .border_radius(f32::MAX);
        Self {
            handle,
            image,
            fallback: fallback.compose(),
        }
    }
}

impl<Message> Meta for Portrait<'_, Message> {}

impl<Message> Widget<Message, Theme, Renderer> for Portrait<'_, Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Fill, Fill)
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.fallback));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        // Measuring starts async decoding on renderers that support it.
        let _ = renderer.measure_image(&self.handle);
        self.fallback
            .layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
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
        // The GPU cache reports 1 × 1 on a decode error. At this point the
        // cache already has the result, including for a genuine 1 × 1 PNG.
        let ready = renderer.measure_image(&self.handle).is_some_and(|size| {
            size != Size::new(1, 1) || renderer.load_image(&self.handle).is_ok()
        });
        if ready {
            <Image as Widget<Message, Theme, Renderer>>::draw(
                &self.image,
                tree,
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        } else if let Some((layout, child)) = layout.iter(&tree.children).next() {
            self.fallback
                .draw(child, renderer, theme, style, layout, cursor, viewport);
        }
    }
}
