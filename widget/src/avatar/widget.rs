//! Avatar painting keeps the outer ring outside the layout footprint.

use iced_core::Length::Fill;
use iced_core::image::{Handle, Renderer as _};
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{
    Background, Border, Color, ContentFit, Element, Event, Layout, Length, Rectangle,
    Renderer as _, Shell, Size, Vector, Widget, layout, mouse, overlay, renderer,
};
use iced_widget::{Image, Renderer};
use icedxdy_core::Theme;
use icedxdy_core::theme::container::{Style, StyleFn};

use super::Geometry;

pub(super) struct Avatar<'a, Message> {
    pub handle: Option<Handle>,
    pub children: Vec<Element<'a, Message, Theme, Renderer>>,
    pub geometry: Geometry,
    pub ring: bool,
    pub style: StyleFn<'a, Theme>,
    pub badge_style: StyleFn<'a, Theme>,
}

impl<Message> Avatar<'_, Message> {
    fn loaded(&self, renderer: &Renderer) -> bool {
        self.handle.as_ref().is_some_and(|handle| {
            renderer.measure_image(handle).is_some_and(|size| {
                // A failed GPU decode reports 1×1. This call only inspects an
                // already-decoded cache entry, including a genuine 1×1 image.
                size != Size::new(1, 1) || renderer.load_image(handle).is_ok()
            })
        })
    }
}

impl<Message> Meta for Avatar<'_, Message> {}

impl<Message> Widget<Message, Theme, Renderer> for Avatar<'_, Message> {
    fn size(&self) -> Size<Length> {
        Size::new(self.geometry.diameter.into(), self.geometry.diameter.into())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, _limits: &layout::Limits) {
        if let Some(handle) = &self.handle {
            let _ = renderer.measure_image(handle);
        }
        let diameter = self.geometry.diameter;
        tree.size = Size::new(diameter, diameter);
        for (index, (child, state)) in self.children.iter_mut().zip(&mut tree.children).enumerate()
        {
            let side = if index == 0 {
                diameter
            } else {
                self.geometry.badge_diameter
            };
            let size = Size::new(side, side);
            child.layout(state, renderer, &layout::Limits::new(size, size));
            state.translation = if index == 0 {
                Vector::ZERO
            } else {
                Vector::new(diameter - side, diameter - side)
            };
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        // WGPU batches quads before images within a layer. Separate each
        // avatar and its foreground so rings/badges cover earlier portraits.
        renderer.start_layer(*viewport);
        if self.ring {
            ring(renderer, theme, bounds);
        }
        let style = (self.style)(theme);
        surface(renderer, style, bounds, false);
        let mut children = layout.iter(&tree.children);
        if let Some((child_layout, child_tree)) = children.next() {
            if let Some(handle) = self.handle.as_ref().filter(|_| self.loaded(renderer)) {
                let image = Image::new(handle.clone())
                    .width(Fill)
                    .height(Fill)
                    .content_fit(ContentFit::Cover)
                    .border_radius(f32::MAX);
                <Image as Widget<Message, Theme, Renderer>>::draw(
                    &image, tree, renderer, theme, inherited, layout, cursor, viewport,
                );
            } else {
                self.children[0].draw(
                    child_tree,
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: style
                            .text_color
                            .map(Into::into)
                            .unwrap_or(inherited.text_color),
                    },
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }
        renderer.end_layer();
        renderer.start_layer(*viewport);
        // The inset edge sits above the portrait, like the source ::after.
        surface(renderer, style, bounds, true);
        if let Some((child_layout, child_tree)) = children.next() {
            let bounds = child_layout.bounds();
            ring(renderer, theme, bounds);
            let style = (self.badge_style)(theme);
            surface(renderer, style, bounds, false);
            self.children[1].draw(
                child_tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style
                        .text_color
                        .map(Into::into)
                        .unwrap_or(inherited.text_color),
                },
                child_layout,
                cursor,
                viewport,
            );
            surface(renderer, style, bounds, true);
        }
        renderer.end_layer();
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let loaded = self.loaded(renderer);
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            for (index, ((layout, tree), child)) in layout
                .iter_mut(&mut tree.children)
                .zip(&mut self.children)
                .enumerate()
            {
                if index != 0 || !loaded {
                    child.operate(tree, layout, viewport, renderer, operation);
                }
            }
        });
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
        let loaded = self.loaded(renderer);
        for (index, ((layout, tree), child)) in layout
            .iter_mut(&mut tree.children)
            .zip(&mut self.children)
            .enumerate()
        {
            if index != 0 || !loaded {
                child.update(tree, event, layout, cursor, renderer, shell, viewport);
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let loaded = self.loaded(renderer);
        layout
            .iter(&tree.children)
            .zip(&self.children)
            .enumerate()
            .filter(|(index, _)| *index != 0 || !loaded)
            .map(|(_, ((layout, tree), child))| {
                child.mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
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
        let loaded = self.loaded(renderer);
        layout
            .iter_mut(&mut tree.children)
            .zip(&mut self.children)
            .enumerate()
            .filter(|(index, _)| *index != 0 || !loaded)
            .flat_map(|(_, ((layout, tree), child))| {
                child.overlay(tree, layout, renderer, viewport, translation, window)
            })
            .collect()
    }
}

fn ring(renderer: &mut Renderer, theme: &Theme, bounds: Rectangle) {
    renderer.fill_quad(
        renderer::Quad {
            bounds: bounds.expand(2.0),
            border: Border::default().rounded(f32::MAX),
            ..renderer::Quad::default()
        },
        iced_core::Color::from(theme.neutral.base),
    );
}

fn surface(renderer: &mut Renderer, style: Style, bounds: Rectangle, edge: bool) {
    let border = if edge {
        style.border.into()
    } else {
        Border::default()
    };
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                radius: f32::MAX.into(),
                ..border
            },
            ..renderer::Quad::default()
        },
        if edge {
            Color::TRANSPARENT.into()
        } else {
            style
                .background
                .map(Background::from)
                .unwrap_or(Color::TRANSPARENT.into())
        },
    );
}
