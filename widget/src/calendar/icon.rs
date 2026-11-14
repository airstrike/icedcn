//! Lucide chevrons inherit the navigation button's text color.
use iced_core::svg::{Handle, Renderer as _, Svg};
use iced_core::widget::{Meta, Tree};
use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use std::sync::LazyLock;
static LEFT: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#)
});
static RIGHT: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#)
});
static DOWN: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#)
});
pub(super) struct Chevron(pub char);
impl Meta for Chevron {}
impl Chevron {
    fn pixels(&self) -> f32 {
        if self.0 == '⌄' { 14. } else { 16. }
    }
}
impl Widget<super::Event, Theme, Renderer> for Chevron {
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
        let handle = match self.0 {
            '‹' => LEFT.clone(),
            '›' => RIGHT.clone(),
            _ => DOWN.clone(),
        };
        renderer.draw_svg(
            Svg::new(handle).color(style.text_color),
            layout.bounds(),
            *viewport,
        );
    }
}
