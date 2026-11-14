use std::f32::consts::PI;
use std::sync::LazyLock;

use iced_core::svg::{Handle, Renderer as _, Svg};
use iced_core::widget::{Meta, Tree};
use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
use iced_widget::Renderer;
use icedxdy_core::Theme;

// Lucide chevron-down, rotated for the open state. Inherit the trigger's
// resolved color so custom themes and disabled opacity reach the icon.
const PATH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;
static DOWN: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(PATH));

pub(super) struct Icon(bool);

impl Icon {
    pub(super) fn new(open: bool) -> Self {
        Self(open)
    }
}

impl Meta for Icon {}

impl<Message> Widget<Message, Theme, Renderer> for Icon {
    fn size(&self) -> Size<Length> {
        Size::new(16.into(), 16.into())
    }

    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(16, 16, Size::ZERO);
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
        let svg = Svg::new(DOWN.clone())
            .color(style.text_color)
            .rotation(if self.0 { PI } else { 0.0 });
        renderer.draw_svg(svg, layout.bounds(), *viewport);
    }
}
