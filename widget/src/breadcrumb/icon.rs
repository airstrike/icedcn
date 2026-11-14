use std::sync::LazyLock;

use iced_core::svg::{Handle, Renderer as _, Svg};
use iced_core::widget::{Meta, Tree};
use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
use iced_widget::Renderer;
use icedcn_core::Theme;

// Lucide chevron-right and ellipsis; resolved text color comes from the parent.
static CHEVRON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#)
});
static ELLIPSIS: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#)
});

pub(super) enum Icon {
    Chevron,
    Ellipsis,
}

impl Icon {
    fn pixels(&self) -> f32 {
        match self {
            Self::Chevron => 14.0,
            Self::Ellipsis => 16.0,
        }
    }
}

impl Meta for Icon {}

impl<Message> Widget<Message, Theme, Renderer> for Icon {
    fn size(&self) -> Size<Length> {
        Size::new(self.pixels().into(), self.pixels().into())
    }

    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(self.pixels(), self.pixels(), Size::ZERO);
    }

    fn draw(
        &self,
        _: &Tree,
        renderer: &mut Renderer,
        _: &Theme,
        style: &renderer::Style,
        layout: Layout,
        _: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let handle = match self {
            Self::Chevron => CHEVRON.clone(),
            Self::Ellipsis => ELLIPSIS.clone(),
        };
        renderer.draw_svg(
            Svg::new(handle).color(style.text_color),
            layout.bounds(),
            *viewport,
        );
    }
}
