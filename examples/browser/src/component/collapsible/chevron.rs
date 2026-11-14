//! The File Tree recipe's 150ms rotation; disclosure content remains instantaneous.
use std::f32::consts::FRAC_PI_2;
use std::sync::LazyLock;
use std::time::Duration;

use iced::time::Instant;
use iced::{Animation, Element, Event, Length, Radians, Rectangle, Renderer, Size};
use iced::{animation::Easing, mouse, window};
use iced_widget::Widget as _;
use iced_widget::core::svg::{Handle, Renderer as _, Svg};
use iced_widget::core::widget::{Meta, Tree, Widget, tree};
use iced_widget::core::{Layout, Shell, layout, renderer};
use icedcn::widget::animation::cubic_bezier;
use icedcn::{Compose, Slot, Theme};

static PATH: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#)
});

pub fn chevron<Message: 'static>(open: bool) -> Slot<'static, Message> {
    Slot::new(Chevron { open })
}

struct Chevron {
    open: bool,
}
struct State {
    turn: Animation<bool>,
    frame: Instant,
}

impl Meta for Chevron {}
impl<Message> Widget<Message, Theme, Renderer> for Chevron {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State {
            turn: Animation::new(self.open)
                .duration(Duration::from_millis(150))
                .easing(Easing::Custom(|t| cubic_bezier(0.4, 0., 0.2, 1., t))),
            frame: Instant::now(),
        })
    }
    fn size(&self) -> Size<Length> {
        Size::new(14.into(), 14.into())
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(14, 14, Size::ZERO);
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
impl<Message> Compose<Message> for Chevron {
    fn compose<'a>(self: Box<Self>) -> Element<'a, Message, Theme>
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
    use super::{Chevron, Duration, FRAC_PI_2, Instant, Renderer, State, Theme, Tree};

    #[test]
    fn rotation_runs_for_150ms_and_reverses_without_jumping() {
        let icon = Chevron { open: false };
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
}
