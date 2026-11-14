use std::f32::consts::FRAC_PI_2;
use std::sync::LazyLock;

use iced_core::svg::{Handle, Renderer as _, Svg};
use iced_core::widget::{Meta, Tree};
use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
use iced_widget::Renderer;
use icedcn_core::Theme;

// Lucide paths inherit the surrounding text color and require no icon font.
static CHEVRON: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#)
});
static REMOVE: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 6-12 12M6 6l12 12"/></svg>"#)
});
static CHECK: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m20 6-11 11-5-5"/></svg>"#)
});
static GLOBE: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M2 12h20M12 2a17 17 0 0 1 0 20 17 17 0 0 1 0-20"/></svg>"#)
});

static SORT: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 16-4 4-4-4m4 4V4M3 8l4-4 4 4M7 4v16"/></svg>"#)
});
static MORE: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#)
});

pub(crate) enum Icon {
    Chevron,
    ChevronRight,
    Remove,
    Check,
    Globe,
    Sort,
    More,
}
impl Icon {
    fn pixels(&self) -> f32 {
        match self {
            Self::More => 12.0,
            _ => 16.0,
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
            Self::Chevron | Self::ChevronRight => CHEVRON.clone(),
            Self::Remove => REMOVE.clone(),
            Self::Check => CHECK.clone(),
            Self::Globe => GLOBE.clone(),
            Self::Sort => SORT.clone(),
            Self::More => MORE.clone(),
        };
        let mut icon = Svg::new(handle).color(style.text_color);
        if matches!(self, Self::ChevronRight) {
            icon = icon.rotation(-FRAC_PI_2);
        }
        renderer.draw_svg(icon, layout.bounds(), *viewport);
    }
}
