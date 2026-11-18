//! The upstream `scroll-fade` utility: a scroll region's edges fade into the page as content
//! scrolls past them.
use iced::Widget as _;
use iced::{Color, Element, Event, Length, Rectangle, Renderer, Size, Vector, mouse};
use iced_widget::core::gradient::Linear;
use iced_widget::core::widget::operation::Scrollable;
use iced_widget::core::widget::{Id, Meta, Operation, Tree, tree};
use iced_widget::core::{Layout, Renderer as _, Shell, Widget, layout, overlay, renderer};
use icedcn::Theme;
use icedcn::widget::animation::cubic_bezier;
use std::f32::consts::PI;

/// `--scroll-fade-reveal`: the scroll distance over which an edge's fade grows to full size.
const REVEAL: f32 = 96.0;

/// Fade the edges of the first scrollable inside `content` into `color`.
///
/// Each edge is `min(12%, 40px)` of the region's height once `REVEAL` pixels of content lie
/// past it, and grows with `ease-in-out` until then, like the CSS scroll timeline. The fade
/// spans only the scrolled content's width and leaves the scrollbar unfaded.
pub(super) fn fade<'a, Message: 'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    color: fn(&Theme) -> Color,
) -> Element<'a, Message, Theme> {
    Fade {
        content: content.into(),
        color,
    }
    .boxed()
}

struct Fade<'a, Message> {
    content: Element<'a, Message, Theme>,
    color: fn(&Theme) -> Color,
}

#[derive(Default)]
struct State {
    offset: f32,
    end: f32,
    width: f32,
}

impl<Message> Meta for Fade<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Fade<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some((child, state)) = layout.iter_mut(&mut tree.children).next() {
            self.content
                .operate(state, child, viewport, renderer, operation);
        }
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
        let Some((child, state)) = layout.iter_mut(&mut tree.children).next() else {
            return;
        };
        self.content
            .update(state, event, child, cursor, renderer, shell, viewport);
        let mut measure = Measure::default();
        self.content
            .operate(state, child, viewport, renderer, &mut measure);
        let fade = tree.state.downcast_mut::<State>();
        if (fade.offset, fade.end, fade.width) != (measure.offset, measure.end, measure.width) {
            fade.offset = measure.offset;
            fade.end = measure.end;
            fade.width = measure.width;
            shell.request_redraw();
        }
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
        let Some((child, state)) = layout.iter(&tree.children).next() else {
            return;
        };
        self.content
            .draw(state, renderer, theme, style, child, cursor, viewport);
        let fade = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let size = (bounds.height * 0.12).min(40.0);
        let ease = |progress: f32| cubic_bezier(0.42, 0.0, 0.58, 1.0, progress);
        let top = size * ease(fade.offset / REVEAL);
        let bottom = size * (1.0 - ease((fade.offset - fade.end + REVEAL) / REVEAL));
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        let solid = (self.color)(theme);
        let clear = solid.scale_alpha(0.0);
        let width = fade.width.min(bounds.width);
        renderer.with_layer(clip, |renderer| {
            for (y, height, start, end) in [
                (bounds.y, top, solid, clear),
                (bounds.y + bounds.height - bottom, bottom, clear, solid),
            ] {
                if height > 0.0 {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: bounds.x,
                                y,
                                width,
                                height,
                            },
                            ..renderer::Quad::default()
                        },
                        Linear::new(PI).add_stop(0.0, start).add_stop(1.0, end),
                    );
                }
            }
        });
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        layout
            .iter(&tree.children)
            .next()
            .map_or(mouse::Interaction::None, |(child, state)| {
                self.content
                    .mouse_interaction(state, child, cursor, viewport, renderer)
            })
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
        match layout.iter_mut(&mut tree.children).next() {
            Some((child, state)) => {
                self.content
                    .overlay(state, child, renderer, viewport, translation, window)
            }
            None => Vec::new(),
        }
    }
}

/// The outermost scrollable's offset, scroll range and content width.
#[derive(Default)]
struct Measure {
    found: bool,
    offset: f32,
    end: f32,
    width: f32,
}

impl Operation for Measure {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        if !self.found {
            f(self);
        }
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        if !self.found {
            self.found = true;
            self.offset = offset.y;
            self.end = (content.height - bounds.height).max(0.0);
            self.width = content.width;
        }
    }
}
