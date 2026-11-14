//! Resolve the panel paint and keyboard ring from the content's focus state.
use crate::container::widget::draw_background;
use iced_core::Widget;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Border, Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedxdy_core::theme::container::{Style, StyleFn};
use icedxdy_core::{Theme, radius};

pub(super) struct Frame<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    actions: bool,
    style: Option<StyleFn<'static, Theme>>,
}
impl<'a, Message> Frame<'a, Message> {
    pub(super) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        style: Option<StyleFn<'static, Theme>>,
    ) -> Self {
        Self {
            content,
            actions: false,
            style,
        }
    }
    pub(super) fn actions(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            content,
            actions: true,
            style: None,
        }
    }
    fn appearance(&self, tree: &Tree, theme: &Theme) -> Option<Style> {
        if self.actions {
            return None;
        }
        Some(if let Some(style) = &self.style {
            style(theme)
        } else {
            let mut style = super::surface(theme);
            if super::viewport::focused(tree) {
                style.border.color = theme.neutral.ring;
            }
            style
        })
    }
}
impl<Message> Meta for Frame<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Frame<'_, Message> {
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
        if self.actions {
            tree.size.width = (tree.size.width - 6.).max(0.);
            tree.size.height = (tree.size.height - 8.).max(0.);
            tree.children[0].translation.y = -4.;
        }
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        op: &mut dyn Operation,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
        self.content.operate(child, layout, viewport, renderer, op);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
        self.content
            .update(child, event, layout, cursor, renderer, shell, viewport);
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
        if !self.actions && super::viewport::focused(tree) {
            let b = layout.bounds();
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: b.x - 3.,
                        y: b.y - 3.,
                        width: b.width + 6.,
                        height: b.height + 6.,
                    },
                    border: Border {
                        width: 3.,
                        radius: (radius::LG + 3.).into(),
                        color: theme.neutral.ring.scale_alpha(0.5).into(),
                    },
                    ..Default::default()
                },
                iced_core::Color::TRANSPARENT,
            );
        }
        let mut inherited = renderer::Style {
            text_color: style.text_color,
        };
        if layout.bounds().intersection(viewport).is_some()
            && let Some(appearance) = self.appearance(tree, theme)
        {
            let paint: iced_widget::container::Style = appearance.into();
            inherited.text_color = paint.text_color.unwrap_or(style.text_color);
            draw_background(renderer, &paint.into(), layout.bounds());
        }
        let (layout, child) = layout.iter(&tree.children).next().expect("frame child");
        self.content
            .draw(child, renderer, theme, &inherited, layout, cursor, viewport);
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, child) = layout.iter(&tree.children).next().expect("frame child");
        self.content
            .mouse_interaction(child, layout, cursor, viewport, renderer)
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("frame child");
        self.content
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::code_block::{Destructive, viewport};
    use iced_core::shell::{Bus, Waker};
    use iced_core::widget::operation::focusable;
    use iced_core::widget::{Id, Tree};
    use iced_core::{Event, Layout, Point, Rectangle, Shell, Size, Widget, layout, mouse, window};
    use iced_widget::Widget as _;
    use iced_widget::{Renderer, Space};
    use icedxdy_core::Theme;
    use icedxdy_core::theme::container::StyleFn;

    fn mounted(style: Option<StyleFn<'static, Theme>>) -> (Frame<'static, ()>, Tree, Renderer) {
        let content = viewport::Viewport::new(
            Space::new().width(100).height(40).boxed(),
            Some(Id::from("code")),
            true,
            false,
            false,
        );
        let mut panel = Frame::new(content.boxed(), style);
        let mut tree = Tree::new(&panel);
        tree.diff(&mut panel);
        let renderer = Renderer::default();
        panel.layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(Size::ZERO, Size::new(260., 100.)),
        );
        (panel, tree, renderer)
    }

    #[test]
    fn keyboard_focus_changes_the_default_border_and_blur_restores_it() {
        for theme in [Theme::light(), Theme::dark()] {
            let (mut panel, mut tree, renderer) = mounted(None);
            let layout = Layout::new(tree.size);
            let viewport = Rectangle::with_size(Size::new(260., 100.));
            let rest = panel.appearance(&tree, &theme).expect("panel paint");
            assert_eq!(rest.border.color, theme.input);
            panel.operate(
                &mut tree,
                layout,
                &viewport,
                &renderer,
                &mut focusable::focus::<()>(Id::from("code")),
            );
            let focused = panel.appearance(&tree, &theme).expect("focused paint");
            assert_eq!(focused.border.color, theme.neutral.ring);
            assert_eq!(focused.background, rest.background);
            assert_eq!(focused.text_color, rest.text_color);
            assert_eq!(focused.border.width, 1.);
            panel.operate(
                &mut tree,
                layout,
                &viewport,
                &renderer,
                &mut focusable::unfocus::<()>(),
            );
            assert_eq!(panel.appearance(&tree, &theme), Some(rest));
            let mut bus = Bus::new();
            let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut bus);
            panel.update(
                &mut tree,
                &Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
                layout,
                mouse::Cursor::Available(Point::new(10., 10.)),
                &renderer,
                &mut shell,
                &viewport,
            );
            assert_eq!(panel.appearance(&tree, &theme), Some(rest), "pointer focus");
        }
    }

    #[test]
    fn focused_destructive_and_custom_panels_preserve_their_complete_paint() {
        for theme in [Theme::light(), Theme::dark()] {
            let destructive: StyleFn<'static, Theme> = Destructive.into();
            let custom: StyleFn<'static, Theme> = Box::new(|theme| {
                let mut style = super::super::surface(theme);
                style.border.color = theme.success.base;
                style.border.width = 2.;
                style.text_color = Some(theme.warning.foreground);
                style
            });
            for style in [destructive, custom] {
                let expected = style(&theme);
                let (mut panel, mut tree, renderer) = mounted(Some(style));
                let layout = Layout::new(tree.size);
                panel.operate(
                    &mut tree,
                    layout,
                    &Rectangle::with_size(Size::new(260., 100.)),
                    &renderer,
                    &mut focusable::focus::<()>(Id::from("code")),
                );
                assert!(viewport::focused(&tree));
                assert_eq!(panel.appearance(&tree, &theme), Some(expected));
            }
        }
    }
}
