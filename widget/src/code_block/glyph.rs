//! Small line icons use the enclosing control's text color.
use crate::compose::Slot;

pub(super) enum Glyph {
    Terminal,
    Copy,
    Check,
}

impl Glyph {
    pub(super) fn view<'a, Message: 'a>(self) -> Slot<'a, Message> {
        #[cfg(feature = "svg")]
        {
            svg::view(self)
        }
        #[cfg(not(feature = "svg"))]
        {
            crate::text::p(match self {
                Self::Terminal => "›",
                Self::Copy => "⧉",
                Self::Check => "✓",
            })
            .size(14.0)
            .line_height(14.0)
            .width(14)
            .align_x(iced_core::Alignment::Center)
            .into()
        }
    }
}

#[cfg(feature = "svg")]
mod svg {
    use super::{Glyph, Slot};
    use std::sync::LazyLock;

    use iced_core::svg::{Handle, Renderer as _, Svg};
    use iced_core::widget::{Meta, Tree};
    use iced_core::{Layout, Length, Rectangle, Size, Widget, layout, mouse, renderer};
    use iced_widget::{Renderer, Widget as _};
    use icedxdy_core::Theme;

    pub(super) fn view<'a, Message: 'a>(glyph: Glyph) -> Slot<'a, Message> {
        glyph.boxed().into()
    }
    fn icon(path: &str) -> Handle {
        Handle::from_memory(format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#).into_bytes())
    }

    impl Meta for Glyph {}

    impl<M> Widget<M, Theme, Renderer> for Glyph {
        fn size(&self) -> Size<Length> {
            Size::new(14.into(), 14.into())
        }

        fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
            tree.size = limits.resolve(14, 14, Size::ZERO);
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
            static TERMINAL: LazyLock<Handle> =
                LazyLock::new(|| icon(r#"<path d="m4 17 6-6-6-6m8 14h8"/>"#));
            static COPY: LazyLock<Handle> = LazyLock::new(|| {
                icon(
                    r#"<rect x="8" y="8" width="14" height="14" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#,
                )
            });
            static CHECK: LazyLock<Handle> =
                LazyLock::new(|| icon(r#"<path d="m20 6-11 11-5-5"/>"#));
            let handle: &Handle = match self {
                Self::Terminal => &TERMINAL,
                Self::Copy => &COPY,
                Self::Check => &CHECK,
            };
            renderer.draw_svg(
                Svg::new(handle.clone()).color(style.text_color),
                layout.bounds(),
                *viewport,
            );
        }
    }
}
