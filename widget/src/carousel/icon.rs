use icedcn_core::Orientation;

use crate::Slot;

#[cfg(feature = "svg")]
use iced_core::svg::{Handle, Renderer as _, Svg};
#[cfg(feature = "svg")]
use iced_core::widget::{Meta, Tree};
#[cfg(feature = "svg")]
use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
#[cfg(feature = "svg")]
use iced_widget::Renderer;
#[cfg(feature = "svg")]
use iced_widget::Widget as _;
#[cfg(feature = "svg")]
use icedcn_core::Theme;
#[cfg(feature = "svg")]
use std::sync::LazyLock;

pub(super) fn chevron<'a, Message: 'a>(axis: Orientation, next: bool) -> Slot<'a, Message> {
    #[cfg(feature = "svg")]
    {
        Icon { axis, next }.boxed().into()
    }
    #[cfg(not(feature = "svg"))]
    {
        crate::text::p(match (axis, next) {
            (Orientation::Horizontal, false) => "‹",
            (Orientation::Horizontal, true) => "›",
            (Orientation::Vertical, false) => "⌃",
            (Orientation::Vertical, true) => "⌄",
        })
        .size(16.0)
        .line_height(iced_core::Pixels(16.0))
        .height(16.0)
        .into()
    }
}

#[cfg(feature = "svg")]
static ICONS: LazyLock<[Handle; 4]> = LazyLock::new(|| {
    ["m15 18-6-6 6-6", "m9 18 6-6-6-6", "m18 15-6-6-6 6", "m6 9 6 6 6-6"]
        .map(|path| Handle::from_memory(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="{path}"/></svg>"#
        ).into_bytes()))
});

#[cfg(feature = "svg")]
struct Icon {
    axis: Orientation,
    next: bool,
}

#[cfg(feature = "svg")]
impl Meta for Icon {}

#[cfg(feature = "svg")]
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
        let index = usize::from(self.axis == Orientation::Vertical) * 2 + usize::from(self.next);
        renderer.draw_svg(
            Svg::new(ICONS[index].clone()).color(style.text_color),
            layout.bounds(),
            *viewport,
        );
    }
}
