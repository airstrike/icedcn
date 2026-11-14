//! Paint-only entrances for newly mounted content.
//!
//! Motion fades inherited text color. Children with their own surface or
//! text colors can supply draw-time paint tokens with [`Motion::theme`].
//! iced's clip layers do not provide group opacity or a blur filter.

use std::slice;
mod preset;
pub use preset::Preset;

use iced_core::animation::Easing;
use iced_core::time::Instant;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Animation, Element, Event, Layout, Length, Rectangle, Renderer, Shell, Size, Vector, Widget,
    layout, mouse, overlay, renderer, window,
};
use icedcn_core::Theme;

use crate::compose::{Compose, Inherited, Slot};

/// An entrance that plays once when its content mounts. Layout and input use
/// the child's final position throughout the transition.
pub struct Motion<'a, Message> {
    content: Slot<'a, Message>,
    preset: Preset,
    animate: bool,
    theme: Option<fn(&Theme, f32) -> Theme>,
}

/// Animate the arrival of a child. In lists, keep stable row keys and use
/// [`Motion::animate`] to distinguish new rows from restored history.
pub fn motion<'a, Message>(content: impl Into<Slot<'a, Message>>) -> Motion<'a, Message> {
    Motion {
        content: content.into(),
        preset: Preset::default(),
        animate: true,
        theme: None,
    }
}

impl<'a, Message> Motion<'a, Message> {
    /// Select the entrance at mount time.
    pub fn preset(mut self, preset: Preset) -> Self {
        self.preset = preset;
        self
    }

    /// Set to false for content that should be displayed immediately on mount.
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Supply paint tokens for a child that draws its own colors. Receives
    /// the surrounding theme and the current opacity during the entrance.
    /// The child's layout and state remain intact.
    pub fn theme(mut self, theme: fn(&Theme, f32) -> Theme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Motion<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, iced_widget::Renderer>
    where
        Self: 'b,
    {
        Surface {
            content: self.content.compose(),
            entrance: self.animate.then_some(self.preset),
            theme: self.theme,
        }
        ._boxed()
    }
}

impl<'a, Message: 'a> From<Motion<'a, Message>> for Slot<'a, Message> {
    fn from(motion: Motion<'a, Message>) -> Self {
        Slot::new(motion)
    }
}

impl<'a, Message: 'a> From<Motion<'a, Message>>
    for Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(motion: Motion<'a, Message>) -> Self {
        Box::new(motion).compose()
    }
}

struct Surface<W, T> {
    content: W,
    entrance: Option<Preset>,
    theme: Option<fn(&T, f32) -> T>,
}

struct Playing {
    preset: Preset,
    timeline: Animation<f32>,
    progress: f32,
}

struct State(Option<Playing>);

impl State {
    fn advance(&mut self, now: Instant) -> bool {
        let Some(play) = &mut self.0 else {
            return false;
        };
        if !play.timeline.is_animating(now) {
            self.0 = None;
            return false;
        }
        play.progress = play.timeline.interpolate(now);
        true
    }
}

impl<W: Meta, T> Meta for Surface<W, T> {
    fn is_void(&self) -> bool {
        self.content.is_void()
    }
}

impl<Message, T, R: Renderer, W: Widget<Message, T, R>> Widget<Message, T, R> for Surface<W, T> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State(self.entrance.map(|preset| {
            Playing {
                preset,
                timeline: Animation::new(0.0)
                    .easing(Easing::Linear)
                    .duration(preset.duration())
                    .go(1.0, Instant::now()),
                progress: 0.0,
            }
        })))
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &layout::Limits) {
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, limits);
        child.translation = Vector::ZERO;
        tree.size = child.size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &R,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event
            && tree.state.downcast_mut::<State>().advance(*now)
            && layout.bounds().intersects(viewport)
        {
            shell.request_redraw();
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
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut R,
        theme: &T,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let child = &tree.children[0];
        if let Some(play) = &tree.state.downcast_ref::<State>().0 {
            let remaining = play.preset.remaining(play.progress);
            let opacity = (1.0 - remaining).clamp(0.0, 1.0);
            if opacity == 0.0 {
                return;
            }
            let transform = play.preset.transform(remaining, layout.bounds());
            let paint = self.theme.map(|paint| paint(theme, opacity));
            let style = renderer::Style {
                text_color: style.text_color.scale_alpha(opacity),
            };
            renderer.with_transformation(transform, |renderer| {
                self.content.draw(
                    child,
                    renderer,
                    paint.as_ref().unwrap_or(theme),
                    &style,
                    layout,
                    cursor,
                    &(*viewport * transform.inverse()),
                );
            });
        } else {
            self.content
                .draw(child, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &R,
        operation: &mut dyn Operation,
    ) {
        self.content
            .operate(&mut tree.children[0], layout, viewport, renderer, operation);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &R,
    ) -> mouse::Interaction {
        self.content
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &R,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, T, R>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::shell::{Bus, Waker};
    use iced_core::window::{Headless, RedrawRequest};
    use iced_widget::Space;
    use std::time::Duration;

    fn row(width: f32, preset: Option<Preset>) -> Element<'static, (), (), ()> {
        Surface {
            content: Space::new().width(width).height(28),
            entrance: preset,
            theme: None,
        }
        ._boxed()
    }

    fn frame(row: &mut Element<'_, (), (), ()>, tree: &mut Tree, now: Instant) -> RedrawRequest {
        let layout = Layout::new(tree.size);
        let mut bus = Bus::new();
        let mut shell = Shell::new(&Headless, Waker::noop(), &mut bus);
        row.update(
            tree,
            &Event::Window(window::Event::RedrawRequested(now)),
            layout,
            mouse::Cursor::Unavailable,
            &(),
            &mut shell,
            &layout.bounds(),
        );
        shell.redraw_request()
    }

    #[test]
    fn rebuilding_a_streaming_row_preserves_its_entrance_and_layout() {
        let mut view = row(120.0, Some(Preset::SlideUp));
        let mut tree = Tree::new(&view);
        tree.diff(&mut view);
        let limits = layout::Limits::new(Size::ZERO, Size::new(600.0, 400.0));
        view.layout(&mut tree, &(), &limits);
        // Drive the native timeline from a known clock.
        let start = Instant::now();
        tree.state
            .downcast_mut::<State>()
            .0
            .as_mut()
            .unwrap()
            .timeline = Animation::new(0.0)
            .easing(Easing::Linear)
            .duration(Preset::SlideUp.duration())
            .go(1.0, start);
        assert_eq!(tree.size, Size::new(120.0, 28.0));
        assert_eq!(frame(&mut view, &mut tree, start), RedrawRequest::NextFrame);
        assert_eq!(tree.size, tree.children[0].size);

        let mut streamed = row(160.0, Some(Preset::Pop));
        tree.diff(&mut streamed);
        streamed.layout(&mut tree, &(), &limits);
        let now = start + Duration::from_millis(130);
        assert_eq!(
            frame(&mut streamed, &mut tree, now),
            RedrawRequest::NextFrame
        );
        let play = tree.state.downcast_ref::<State>().0.as_ref().unwrap();
        assert_eq!(play.preset, Preset::SlideUp);
        assert_eq!(play.progress, 0.5);
        assert_eq!(tree.size, Size::new(160.0, 28.0));
        assert_eq!(tree.children[0].translation, Vector::ZERO);

        assert_eq!(
            frame(&mut streamed, &mut tree, start + Preset::SlideUp.duration()),
            RedrawRequest::Wait
        );
        tree.diff(&mut row(180.0, Some(Preset::SlideSide)));
        assert!(tree.state.downcast_ref::<State>().0.is_none());
    }

    #[test]
    fn history_and_completed_presets_stay_at_the_final_position() {
        let mut history = row(120.0, None);
        let mut tree = Tree::new(&history);
        tree.diff(&mut history);
        tree.diff(&mut row(160.0, Some(Preset::Pop)));
        assert!(tree.state.downcast_ref::<State>().0.is_none());

        for preset in Preset::ALL {
            assert_eq!(preset.remaining(0.0), 1.0);
            assert_eq!(preset.remaining(1.0), 0.0);
        }
    }
}
