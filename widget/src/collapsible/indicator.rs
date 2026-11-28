//! The disclosure indicator keeps its rotation in widget tree state.
use std::f32::consts::FRAC_PI_2;
use std::sync::LazyLock;
use std::time::Duration;

use iced_core::svg::{Handle, Renderer as _, Svg};
use iced_core::time::Instant;
use iced_core::widget::{Meta, Tree, Widget, tree};
use iced_core::{Animation, Element, Event, Length, Pixels, Radians, Rectangle, Size};
use iced_core::{Layout, Shell, layout, renderer};
use iced_core::{animation::Easing, mouse, window};
use iced_widget::Renderer;
use iced_widget::Widget as _;
use icedcn_core::Theme;

use crate::animation::cubic_bezier;
use crate::{Compose, View};

static PATH: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#)
});

pub(super) fn indicator(open: bool) -> Indicator {
    Indicator {
        open,
        size: 16.,
        duration: Duration::from_millis(150),
    }
}

/// A disclosure chevron, constructed by [`super::Handle::indicator`].
pub struct Indicator {
    open: bool,
    size: f32,
    duration: Duration,
}

impl Indicator {
    /// Set the icon's square bounds in pixels.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0.max(0.);
        self
    }

    /// Set the rotation duration. Use `Duration::ZERO` for immediate changes.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

struct State {
    turn: Animation<bool>,
    frame: Instant,
}

impl Meta for Indicator {}
impl<Message> Widget<Message, Theme, Renderer> for Indicator {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State {
            turn: Animation::new(self.open)
                .duration(self.duration)
                .easing(Easing::Custom(|t| cubic_bezier(0.4, 0., 0.2, 1., t))),
            frame: Instant::now(),
        })
    }
    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        state.turn = state.turn.clone().duration(self.duration);
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.size.into(), self.size.into())
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(self.size, self.size, Size::ZERO);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _: Layout,
        _: mouse::Cursor,
        _: &Renderer,
        shell: &mut Shell<'_, Message>,
        _: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<State>();
            if state.advance(self.open, *now) {
                shell.request_redraw();
            }
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _: &Theme,
        style: &renderer::Style,
        layout: Layout,
        _: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let angle = state.turn.interpolate(0., FRAC_PI_2, state.frame);
        renderer.draw_svg(
            Svg::new(PATH.clone())
                .color(style.text_color)
                .rotation(Radians(angle)),
            layout.bounds(),
            *viewport,
        );
    }
}
impl<'a, Message: 'a> From<Indicator> for View<'a, Message> {
    fn from(indicator: Indicator) -> Self {
        View::new(indicator)
    }
}

impl<'a, Message: 'a> From<Indicator> for Element<'a, Message, Theme, Renderer> {
    fn from(indicator: Indicator) -> Self {
        Box::new(indicator).compose()
    }
}

impl<Message> Compose<Message> for Indicator {
    fn compose<'a>(self: Box<Self>) -> Element<'a, Message, Theme, Renderer>
    where
        Self: 'a,
    {
        (*self).boxed()
    }
}

impl State {
    fn advance(&mut self, open: bool, now: Instant) -> bool {
        self.frame = now;
        if self.turn.value() != open {
            self.turn.go_mut(open, now);
        }
        self.turn.is_animating(now)
    }
}

#[cfg(test)]
mod tests {
    use super::{Duration, FRAC_PI_2, Instant, Renderer, State, Theme, Tree};

    #[test]
    fn rotation_runs_for_150ms_and_reverses_without_jumping() {
        let icon = super::indicator(false);
        let mut tree = Tree::new::<(), Theme, Renderer>(&icon);
        let state = tree.state.downcast_mut::<State>();
        let now = Instant::now();
        assert!(state.advance(true, now));
        assert_eq!(state.turn.interpolate(0., FRAC_PI_2, now), 0.);
        let middle = now + Duration::from_millis(75);
        assert!(state.advance(true, middle));
        let angle = state.turn.interpolate(0., FRAC_PI_2, middle);
        assert!(angle > 0. && angle < FRAC_PI_2);
        assert!(!state.advance(true, now + Duration::from_millis(150)));
        assert!((state.turn.interpolate(0., FRAC_PI_2, state.frame) - FRAC_PI_2).abs() < 0.0001);
        assert!(state.advance(false, state.frame));
        assert!((state.turn.interpolate(0., FRAC_PI_2, state.frame) - FRAC_PI_2).abs() < 0.0001);
        assert!(!state.advance(false, now + Duration::from_millis(300)));
        assert!(state.turn.interpolate(0., FRAC_PI_2, state.frame).abs() < 0.0001);
    }

    #[test]
    fn rebuilding_preserves_rotation_and_accepts_immediate_motion() {
        let icon = super::indicator(false);
        let mut tree = Tree::new::<(), Theme, Renderer>(&icon);
        let now = Instant::now();
        tree.state.downcast_mut::<State>().advance(true, now);
        let mut next = super::indicator(true).size(14);
        iced_core::Widget::<(), Theme, Renderer>::diff(&mut next, &mut tree);
        let state = tree.state.downcast_mut::<State>();
        assert!(state.advance(true, now + Duration::from_millis(75)));
        let angle = state.turn.interpolate(0., FRAC_PI_2, state.frame);
        assert!(angle > 0. && angle < FRAC_PI_2);
        let mut immediate = super::indicator(false).duration(Duration::ZERO);
        iced_core::Widget::<(), Theme, Renderer>::diff(&mut immediate, &mut tree);
        let state = tree.state.downcast_mut::<State>();
        assert!(!state.advance(false, now + Duration::from_millis(100)));
        assert_eq!(state.turn.interpolate(0., FRAC_PI_2, state.frame), 0.);
    }
}
