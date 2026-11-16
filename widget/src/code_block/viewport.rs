//! Focus and keyboard navigation around the native scrollable; scrolling stays native.
use std::any::Any;

use iced_core::Widget;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::operation::scrollable::AbsoluteOffset;
use iced_core::widget::operation::{self, Focusable, Scrollable};
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Id, Meta, Operation};
use iced_core::{
    Color, Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

use super::editor::Reveal;

pub(super) struct Viewport<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    id: Option<Id>,
    focusable: bool,
    collapsed: bool,
    vertical_fade: bool,
}
impl<'a, Message> Viewport<'a, Message> {
    pub(super) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        id: Option<Id>,
        focusable: bool,
        collapsed: bool,
        vertical_fade: bool,
    ) -> Self {
        Self {
            content,
            id,
            focusable,
            collapsed,
            vertical_fade,
        }
    }
}
#[derive(Default)]
struct State {
    focused: bool,
    keyboard: bool,
    metrics: Metrics,
}
impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }
    fn unfocus(&mut self) {
        self.focused = false;
    }
}
pub(super) fn focused(tree: &Tree) -> bool {
    (tree.tag == tree::Tag::of::<State>() && {
        let s = tree.state.downcast_ref::<State>();
        s.focused && s.keyboard
    }) || super::editor::focused(tree)
        || tree.children.iter().any(focused)
}
#[derive(Default, Clone, Copy)]
struct Metrics {
    offset: Vector,
    end: Vector,
}
#[derive(Default)]
struct Scroll {
    metrics: Metrics,
    key: Option<Named>,
    vertical: bool,
    reveal: Option<AbsoluteOffset<Option<f32>>>,
    found: bool,
}
impl Operation for Scroll {
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
        state: &mut dyn Scrollable,
    ) {
        let end = Vector::new(
            (content.width - bounds.width).max(0.),
            (content.height - bounds.height).max(0.),
        );
        self.found = true;
        self.metrics = Metrics { offset, end };
        let animation = operation::Animation::Instant;
        if let Some(offset) = self.reveal {
            state.scroll_to(offset, animation, bounds, content);
            return;
        }
        let delta = match self.key {
            Some(Named::ArrowLeft) => Vector::new(-40., 0.),
            Some(Named::ArrowRight) => Vector::new(40., 0.),
            Some(Named::ArrowUp) if self.vertical => Vector::new(0., -40.),
            Some(Named::ArrowDown) if self.vertical => Vector::new(0., 40.),
            Some(Named::PageUp) if self.vertical => Vector::new(0., -bounds.height * 0.9),
            Some(Named::PageDown | Named::Space) if self.vertical => {
                Vector::new(0., bounds.height * 0.9)
            }
            Some(Named::Home) => {
                state.scroll_to(
                    AbsoluteOffset {
                        x: Some(0.),
                        y: self.vertical.then_some(0.),
                    },
                    animation,
                    bounds,
                    content,
                );
                return;
            }
            Some(Named::End) => {
                state.scroll_to(
                    AbsoluteOffset {
                        x: (!self.vertical || end.y == 0.).then_some(end.x),
                        y: self.vertical.then_some(end.y),
                    },
                    animation,
                    bounds,
                    content,
                );
                return;
            }
            _ => return,
        };
        // Relative operations use the scrollable's current offset even when
        // several key presses arrive before the next redraw.
        state.scroll_by(
            AbsoluteOffset {
                x: delta.x,
                y: delta.y,
            },
            animation,
            bounds,
            content,
        );
    }
}
/// Finds a moved editor caret and where the scrollable shows its content.
#[derive(Default)]
struct Caret {
    viewport: Option<(Rectangle, Vector)>,
    caret: Option<Rectangle>,
}
impl Operation for Caret {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        f(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        _: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.viewport.get_or_insert((bounds, offset));
    }
    fn custom(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if let Some(reveal) = state.downcast_mut::<Reveal>()
            && reveal.take()
        {
            self.caret = Some(bounds);
        }
    }
}
impl Caret {
    /// The offset that shows the caret, or `None` when it already shows.
    fn offset(&self) -> Option<AbsoluteOffset<Option<f32>>> {
        let ((bounds, offset), caret) = (self.viewport?, self.caret?);
        // Near the start, show the start (the line numbers) rather than the
        // caret flush against the edge.
        let reveal = |start: f32, length: f32, offset: f32, size: f32| {
            let end = start + length;
            if start >= offset && end <= offset + size {
                None
            } else if end <= size {
                Some(0.)
            } else if start < offset {
                Some(start)
            } else {
                Some(end - size)
            }
        };
        let x = reveal(caret.x - bounds.x, caret.width, offset.x, bounds.width);
        let y = reveal(caret.y - bounds.y, caret.height, offset.y, bounds.height);
        (x.is_some() || y.is_some()).then_some(AbsoluteOffset { x, y })
    }
}

impl<Message> Meta for Viewport<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Viewport<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
        if !self.focusable {
            tree.state.downcast_mut::<State>().focused = false;
        }
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
        let bounds = Rectangle::with_size(tree.size);
        let mut caret = Caret::default();
        self.content.operate(
            &mut tree.children[0],
            Layout::new(tree.size),
            &bounds,
            renderer,
            &mut caret,
        );
        if let Some(offset) = caret.offset() {
            self.content.operate(
                &mut tree.children[0],
                Layout::new(tree.size),
                &bounds,
                renderer,
                &mut Scroll {
                    reveal: Some(offset),
                    ..Scroll::default()
                },
            );
        }
        let mut scroll = Scroll::default();
        self.content.operate(
            &mut tree.children[0],
            Layout::new(tree.size),
            &Rectangle::with_size(tree.size),
            renderer,
            &mut scroll,
        );
        tree.state.downcast_mut::<State>().metrics = scroll.metrics;
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.focusable {
            operation.focusable(
                self.id.as_ref(),
                layout.bounds(),
                tree.state.downcast_mut::<State>(),
            );
        }
        self.content
            .operate(&mut tree.children[0], layout, viewport, renderer, operation);
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
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            let state = tree.state.downcast_mut::<State>();
            state.focused =
                self.focusable && cursor.is_over(layout.bounds()) && !shell.is_event_captured();
            state.keyboard = false;
        }
        if tree.state.downcast_ref::<State>().focused
            && !shell.is_event_captured()
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key),
                ..
            }) = event
            && matches!(
                key,
                Named::ArrowLeft
                    | Named::ArrowRight
                    | Named::ArrowUp
                    | Named::ArrowDown
                    | Named::PageUp
                    | Named::PageDown
                    | Named::Home
                    | Named::End
                    | Named::Space
            )
        {
            let mut scroll = Scroll {
                key: Some(*key),
                vertical: !self.collapsed,
                ..Scroll::default()
            };
            self.content.operate(
                &mut tree.children[0],
                layout,
                viewport,
                renderer,
                &mut scroll,
            );
            tree.state.downcast_mut::<State>().keyboard = true;
            shell.capture_event();
            shell.request_redraw();
            return;
        }
        // A collapsed panel clips the complete listing. Wheel input continues to the document.
        if self.collapsed && matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. })) {
            return;
        }
        self.content.update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
        let mut scroll = Scroll::default();
        self.content.operate(
            &mut tree.children[0],
            layout,
            viewport,
            renderer,
            &mut scroll,
        );
        tree.state.downcast_mut::<State>().metrics = scroll.metrics;
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
        self.content.draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
        let Some(clip) = layout.bounds().intersection(viewport) else {
            return;
        };
        let metrics = tree.state.downcast_ref::<State>().metrics;
        let bounds = layout.bounds();
        let base: Color = theme.neutral.base.into();
        // Paint only at the remaining scroll edges, preserving pointer input and text selection.
        renderer.with_layer(clip, |renderer| {
            for (active, rect, reverse, vertical) in [
                (
                    metrics.offset.x > 0.5,
                    Rectangle {
                        width: 20.,
                        ..bounds
                    },
                    false,
                    false,
                ),
                (
                    metrics.offset.x + 0.5 < metrics.end.x,
                    Rectangle {
                        x: bounds.x + bounds.width - 20.,
                        width: 20.,
                        ..bounds
                    },
                    true,
                    false,
                ),
                (
                    self.vertical_fade && metrics.offset.y > 0.5,
                    Rectangle {
                        height: 20.,
                        ..bounds
                    },
                    false,
                    true,
                ),
                (
                    self.vertical_fade && metrics.offset.y + 0.5 < metrics.end.y,
                    Rectangle {
                        y: bounds.y + bounds.height - 20.,
                        height: 20.,
                        ..bounds
                    },
                    true,
                    true,
                ),
            ] {
                if !active {
                    continue;
                }
                for n in 0..20 {
                    let alpha = if reverse {
                        n as f32 / 20.
                    } else {
                        1. - n as f32 / 20.
                    };
                    let band = if vertical {
                        Rectangle {
                            y: rect.y + n as f32,
                            height: 1.,
                            ..rect
                        }
                    } else {
                        Rectangle {
                            x: rect.x + n as f32,
                            width: 1.,
                            ..rect
                        }
                    };
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: band,
                            ..Default::default()
                        },
                        base.scale_alpha(alpha),
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
        self.content
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
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
        self.content.overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
