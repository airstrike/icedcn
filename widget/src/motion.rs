//! Paint-only entrances for newly mounted content.

use std::time::Duration;
use std::{fmt, slice};

use iced_core::time::Instant;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Animation, Element, Event, Layout, Length, Rectangle, Renderer, Shell, Size, Transformation,
    Vector, Widget, layout, mouse, overlay, renderer, window,
};
use icedxdy_core::Theme;

use crate::compose::{Compose, Inherited, Slot};

/// Portable paint transforms for the arrival of a widget.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    /// Move into place from just below.
    #[default]
    SlideUp,
    /// Move into place from the right.
    SlideSide,
    /// Scale up from the center while rising slightly.
    Pop,
    /// Settle into place with a damped vertical bounce.
    SpringBounce,
}

impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SlideUp => "Slide Up",
            Self::SlideSide => "Slide Side",
            Self::Pop => "Pop",
            Self::SpringBounce => "Spring Bounce",
        })
    }
}

impl Preset {
    /// All presets in picker order.
    pub const ALL: [Self; 4] = [
        Self::SlideUp,
        Self::SlideSide,
        Self::Pop,
        Self::SpringBounce,
    ];

    fn duration(self) -> Duration {
        match self {
            Self::SpringBounce => Duration::from_millis(320),
            _ => Duration::from_millis(200),
        }
    }

    fn transform(self, progress: f32, bounds: Rectangle) -> Transformation {
        if progress >= 1.0 {
            return Transformation::IDENTITY;
        }
        let remaining = 1.0 - crate::animation::ease_out(progress);
        let (x, y, scale) = match self {
            Self::SlideUp => (0.0, 10.0 * remaining, 1.0),
            Self::SlideSide => (18.0 * remaining, 0.0, 1.0),
            Self::Pop => (0.0, 6.0 * remaining, 1.0 - 0.06 * remaining),
            Self::SpringBounce => {
                let oscillation = (progress * 9.0).cos() * remaining;
                (0.0, 9.0 * oscillation, 1.0 - 0.08 * oscillation)
            }
        };
        let center = bounds.center();
        Transformation::translate(center.x + x, center.y + y)
            * Transformation::scale(scale)
            * Transformation::translate(-center.x, -center.y)
    }
}

/// An entrance that plays once when its content mounts. Layout and input use
/// the child's final position throughout the transition.
pub struct Motion<'a, Message> {
    content: Slot<'a, Message>,
    preset: Preset,
    animate: bool,
}

/// Animate the arrival of a child. In lists, keep stable row keys and use
/// [`Motion::animate`] to distinguish new rows from restored history.
pub fn motion<'a, Message>(content: impl Into<Slot<'a, Message>>) -> Motion<'a, Message> {
    Motion {
        content: content.into(),
        preset: Preset::default(),
        animate: true,
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

struct Surface<W> {
    content: W,
    entrance: Option<Preset>,
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

impl<W: Meta> Meta for Surface<W> {
    fn is_void(&self) -> bool {
        self.content.is_void()
    }
}

impl<Message, T, R: Renderer, W: Widget<Message, T, R>> Widget<Message, T, R> for Surface<W> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State(self.entrance.map(|preset| {
            Playing {
                preset,
                timeline: Animation::new(0.0)
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
            let transform = play.preset.transform(play.progress, layout.bounds());
            renderer.with_transformation(transform, |renderer| {
                self.content.draw(
                    child,
                    renderer,
                    theme,
                    style,
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

    fn row(width: f32, preset: Option<Preset>) -> Element<'static, (), (), ()> {
        Surface {
            content: Space::new().width(width).height(28),
            entrance: preset,
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
            .duration(Preset::SlideUp.duration())
            .go(1.0, start);
        assert_eq!(tree.size, Size::new(120.0, 28.0));
        assert_eq!(frame(&mut view, &mut tree, start), RedrawRequest::NextFrame);
        assert_eq!(tree.size, tree.children[0].size);

        let mut streamed = row(160.0, Some(Preset::Pop));
        tree.diff(&mut streamed);
        streamed.layout(&mut tree, &(), &limits);
        let now = start + Duration::from_millis(100);
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

        let bounds = Rectangle::new(iced_core::Point::new(8.0, 24.0), Size::new(120.0, 28.0));
        for preset in Preset::ALL {
            assert_ne!(preset.transform(0.0, bounds), Transformation::IDENTITY);
            assert_eq!(preset.transform(1.0, bounds), Transformation::IDENTITY);
        }
    }
}
