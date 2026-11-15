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
static PANEL_LEFT: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 3v18"/></svg>"#)
});
static CALENDAR: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 2v4m8-4v4M3 10h18"/><rect width="18" height="18" x="3" y="4" rx="2"/></svg>"#)
});
static SEARCH: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>"#)
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

static CIRCLE_CHECK: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#)
});
static INFO: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#)
});
static TRIANGLE_ALERT: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m10.3 3.9-8.2 14.2A2 2 0 0 0 3.8 21h16.4a2 2 0 0 0 1.7-2.9L13.7 3.9a2 2 0 0 0-3.4 0ZM12 9v4m0 4h.01"/></svg>"#)
});
static OCTAGON_X: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8.7 2h6.6L22 8.7v6.6L15.3 22H8.7L2 15.3V8.7L8.7 2Zm.3 7 6 6m0-6-6 6"/></svg>"#)
});

pub(crate) enum Icon {
    CircleCheck,
    Info,
    TriangleAlert,
    OctagonX,

    Chevron,
    ChevronRight,
    Calendar,
    PanelLeft,
    Remove,
    Check,
    Search,
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
            Self::CircleCheck => CIRCLE_CHECK.clone(),
            Self::Info => INFO.clone(),
            Self::TriangleAlert => TRIANGLE_ALERT.clone(),
            Self::OctagonX => OCTAGON_X.clone(),
            Self::Remove => REMOVE.clone(),
            Self::Check => CHECK.clone(),
            Self::Search => SEARCH.clone(),
            Self::Calendar => CALENDAR.clone(),
            Self::PanelLeft => PANEL_LEFT.clone(),
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
