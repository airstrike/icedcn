//! Direct-child geometry shared by grouped control surfaces.
use iced_core::border::Radius;
use iced_core::{Background, Padding, Rectangle, Renderer, renderer};
use icedcn_core::Orientation;

/// A direct child's position in a button group. Consumed by its control surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Join {
    pub(crate) orientation: Orientation,
    pub(crate) first: bool,
    pub(crate) last: bool,
}

impl Join {
    pub(crate) fn radius(self, mut radius: Radius) -> Radius {
        // A group keeps Nova's rounded-lg outer contour, including small controls.
        radius.top_left = radius.top_left.max(10.);
        radius.top_right = radius.top_right.max(10.);
        radius.bottom_left = radius.bottom_left.max(10.);
        radius.bottom_right = radius.bottom_right.max(10.);
        match self.orientation {
            Orientation::Horizontal => {
                if !self.first {
                    radius.top_left = 0.;
                    radius.bottom_left = 0.;
                }
                if !self.last {
                    radius.top_right = 0.;
                    radius.bottom_right = 0.;
                }
            }
            Orientation::Vertical => {
                if !self.first {
                    radius.top_left = 0.;
                    radius.top_right = 0.;
                }
                if !self.last {
                    radius.bottom_left = 0.;
                    radius.bottom_right = 0.;
                }
            }
        }
        radius
    }
    pub(crate) fn padding(self, mut padding: Padding, width: f32) -> Padding {
        if !self.first {
            match self.orientation {
                Orientation::Horizontal => padding.left = (padding.left - width).max(0.),
                Orientation::Vertical => padding.top = (padding.top - width).max(0.),
            }
        }
        padding
    }
}

/// Omit the inward border and keep content above the surface's clip layer.
pub(crate) fn paint<R: Renderer>(
    renderer: &mut R,
    mut quad: renderer::Quad,
    background: impl Into<Background>,
    join: Option<Join>,
    viewport: Rectangle,
    content: impl FnOnce(&mut R),
) {
    let background = background.into();
    if let Some(join) = join {
        quad.border.radius = join.radius(quad.border.radius);
        if !join.first && quad.border.width > 0. {
            let bounds = quad.bounds;
            match join.orientation {
                Orientation::Horizontal => {
                    quad.bounds.x -= quad.border.width;
                    quad.bounds.width += quad.border.width;
                }
                Orientation::Vertical => {
                    quad.bounds.y -= quad.border.width;
                    quad.bounds.height += quad.border.width;
                }
            }
            renderer.with_layer(bounds, |renderer| renderer.fill_quad(quad, background));
            // Drawing back into the parent layer would put text and icons behind
            // the clipped quad. A sibling layer keeps the caller's full viewport.
            renderer.with_layer(viewport, content);
            return;
        }
    }
    renderer.fill_quad(quad, background);
    content(renderer);
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Border, Color, Rectangle, Size, Transformation, image};
    #[derive(Default)]
    struct Recording {
        layers: Vec<Rectangle>,
        quads: Vec<(renderer::Quad, Option<Rectangle>)>,
    }
    impl Renderer for Recording {
        fn start_layer(&mut self, bounds: Rectangle) {
            self.layers.push(bounds);
        }
        fn end_layer(&mut self) {
            self.layers.pop();
        }
        fn start_transformation(&mut self, _: Transformation) {}
        fn end_transformation(&mut self) {}
        fn fill_quad(&mut self, quad: renderer::Quad, _: impl Into<Background>) {
            self.quads.push((quad, self.layers.last().copied()));
        }
        fn allocate_image(
            &self,
            _: &image::Handle,
            _: impl FnOnce(Result<image::Allocation, image::Error>) + Send + 'static,
        ) {
        }
        fn hint(&mut self, _: renderer::Scale) {}
        fn scale(&self) -> Option<renderer::Scale> {
            None
        }
        fn reset(&mut self, _: Rectangle) {
            self.quads.clear();
        }
        fn settings(&self) -> renderer::Settings {
            renderer::Settings::default()
        }
    }
    #[test]
    fn incoming_border_is_outside_surface_clip_but_following_focus_paint_is_unclipped() {
        for orientation in [Orientation::Horizontal, Orientation::Vertical] {
            let mut renderer = Recording::default();
            let bounds = Rectangle::new([32., 32.].into(), Size::new(48., 32.));
            let surface = renderer::Quad {
                bounds,
                border: Border {
                    width: 1.,
                    radius: 17.into(),
                    color: Color::BLACK,
                },
                ..renderer::Quad::default()
            };
            let viewport = bounds.expand(20.);
            paint(
                &mut renderer,
                surface,
                Color::WHITE,
                Some(Join {
                    orientation,
                    first: false,
                    last: true,
                }),
                viewport,
                |renderer| renderer.fill_quad(surface, Color::BLACK),
            );
            // A focus halo following surface paint must not inherit its clip.
            renderer.fill_quad(
                renderer::Quad {
                    bounds: bounds.expand(3.),
                    ..surface
                },
                Color::TRANSPARENT,
            );
            let (painted, clip) = renderer.quads[0];
            assert_eq!(clip, Some(bounds));
            match orientation {
                Orientation::Horizontal => {
                    assert_eq!(painted.bounds.x + painted.border.width, bounds.x);
                    assert_eq!(painted.border.radius.top_right, 17.);
                    assert_eq!(painted.border.radius.top_left, 0.);
                }
                Orientation::Vertical => {
                    assert_eq!(painted.bounds.y + painted.border.width, bounds.y);
                    assert_eq!(painted.border.radius.bottom_right, 17.);
                    assert_eq!(painted.border.radius.top_left, 0.);
                }
            }
            assert_eq!(renderer.quads[1].1, Some(viewport));
            assert_eq!(renderer.quads[2].1, None);
        }
    }
    #[test]
    fn standalone_and_single_surface_preserve_custom_paint() {
        for join in [
            None,
            Some(Join {
                orientation: Orientation::Horizontal,
                first: true,
                last: true,
            }),
        ] {
            let mut renderer = Recording::default();
            let quad = renderer::Quad {
                bounds: Rectangle::with_size(Size::new(45., 30.)),
                border: Border {
                    width: 2.,
                    radius: 17.into(),
                    color: Color::BLACK,
                },
                ..renderer::Quad::default()
            };
            paint(&mut renderer, quad, Color::WHITE, join, quad.bounds, |_| {});
            assert_eq!(renderer.quads, [(quad, None)]);
        }
    }
}
