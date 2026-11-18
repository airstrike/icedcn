//! A scrollable's visibility modes, read through the redraws it schedules: a shown scrollbar
//! that may hide asks for a frame when its hide delay ends, then hides at once or, with a fade,
//! over the frames after it. A hidden or steady one asks for nothing.
use iced_core::renderer::Headless;
use iced_core::time::{Duration, Instant};
use iced_core::window::RedrawRequest;
use iced_core::{Element, Event, Point, Size, mouse, renderer, shell, window};
use iced_runtime::user_interface::{self, Cache, UserInterface};
use iced_test::futures::futures::executor::block_on;
use iced_widget::{Renderer, column, space};
use icedcn_core::Theme;
use icedcn_widget::scrollable::{self, FADE, HIDE_DELAY, Visibility};

const INSIDE: Point = Point::new(40.0, 50.0);
const OUTSIDE: Point = Point::new(150.0, 150.0);
/// On the default thin vertical scrollbar, which spans the last 2px of the 100px viewport.
const ON_BAR: Point = Point::new(99.0, 5.0);
/// Radix's scroll-end debounce.
const SCROLL_END: Duration = Duration::from_millis(100);

struct Harness {
    visibility: Visibility,
    delay: Duration,
    fade: Duration,
    renderer: Renderer,
    cache: Option<Cache>,
    cursor: Point,
    start: Instant,
}

impl Harness {
    fn new(visibility: Visibility) -> Self {
        Harness::with_delay(visibility, HIDE_DELAY)
    }

    fn fading(visibility: Visibility) -> Self {
        Harness::with(visibility, HIDE_DELAY, FADE)
    }

    fn with_delay(visibility: Visibility, delay: Duration) -> Self {
        Harness::with(visibility, delay, Duration::ZERO)
    }

    fn with(visibility: Visibility, delay: Duration, fade: Duration) -> Self {
        let renderer = block_on(Renderer::new(renderer::Settings::default(), None))
            .expect("the null renderer always builds");
        let mut harness = Harness {
            visibility,
            delay,
            fade,
            renderer,
            cache: Some(Cache::default()),
            cursor: OUTSIDE,
            start: Instant::now(),
        };
        // The first frame settles the initial offset.
        assert_eq!(harness.redraw(0), RedrawRequest::Wait);
        harness
    }

    fn view(&self) -> Element<'static, (), Theme, Renderer> {
        scrollable::scrollable(column![space().height(400)].width(100))
            .width(100)
            .height(100)
            .visibility(self.visibility)
            .hide_delay(self.delay)
            .fade(self.fade)
            .into()
    }

    fn update(&mut self, events: &[Event]) -> RedrawRequest {
        let cache = self
            .cache
            .take()
            .expect("the cache returns after each update");
        let mut ui = UserInterface::build(
            self.view(),
            Size::new(200.0, 200.0),
            cache,
            &mut self.renderer,
        );
        let (state, _) = ui.update(
            &window::Headless,
            &shell::Waker::noop(),
            events,
            mouse::Cursor::Available(self.cursor),
            &mut self.renderer,
            &mut shell::Bus::new(),
        );
        self.cache = Some(ui.into_cache());
        match state {
            user_interface::State::Updated { redraw_request, .. } => redraw_request,
            user_interface::State::Outdated => RedrawRequest::NextFrame,
        }
    }

    fn at(&self, millis: u64) -> Instant {
        self.start + Duration::from_millis(millis)
    }

    /// Draw the frame `millis` after the start.
    fn redraw(&mut self, millis: u64) -> RedrawRequest {
        let now = self.at(millis);
        self.update(&[Event::Window(window::Event::RedrawRequested(now))])
    }

    fn point(&mut self, position: Point) -> RedrawRequest {
        self.cursor = position;
        self.update(&[Event::Mouse(mouse::Event::CursorMoved { position })])
    }

    fn wheel(&mut self) -> RedrawRequest {
        self.update(&[Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x: 0.0, y: -50.0 },
        })])
    }

    /// Draw the frame where the hide delay ends at `millis`, and any fade after it, and expect
    /// the scrollbar to end hidden.
    fn hide(&mut self, millis: u64) {
        if self.fade.is_zero() {
            assert_eq!(
                self.redraw(millis),
                RedrawRequest::Wait,
                "it hides at once and asks for no fade frames"
            );
            assert_eq!(self.redraw(millis + 100), RedrawRequest::Wait);
            return;
        }
        let fade = self.fade.as_millis() as u64;
        assert_eq!(
            self.redraw(millis),
            RedrawRequest::NextFrame,
            "the fade starts"
        );
        assert_eq!(
            self.redraw(millis + fade / 2),
            RedrawRequest::NextFrame,
            "the fade runs frame by frame"
        );
        assert_eq!(
            self.redraw(millis + fade),
            RedrawRequest::Wait,
            "a hidden scrollbar asks for no frames"
        );
    }
}

fn millis(duration: Duration) -> u64 {
    duration.as_millis() as u64
}

#[test]
fn hover_shows_on_enter_and_fades_after_the_delay() {
    let mut ui = Harness::new(Visibility::Hover);
    assert_eq!(ui.redraw(16), RedrawRequest::Wait, "hidden while away");

    let _ = ui.point(INSIDE);
    assert_eq!(ui.redraw(32), RedrawRequest::Wait, "shown while over it");
    let _ = ui.wheel();
    let _ = ui.redraw(48);
    assert_eq!(
        ui.redraw(2_000),
        RedrawRequest::Wait,
        "a pointer over it holds it"
    );

    let _ = ui.point(OUTSIDE);
    assert_eq!(
        ui.redraw(3_000),
        RedrawRequest::At(ui.at(3_000) + HIDE_DELAY),
        "leaving starts the hide delay"
    );
    assert_eq!(
        ui.redraw(3_100),
        RedrawRequest::At(ui.at(3_000) + HIDE_DELAY)
    );
    ui.hide(3_000 + millis(HIDE_DELAY));
    assert_eq!(ui.redraw(5_000), RedrawRequest::Wait);
}

#[test]
fn hover_reentering_cancels_the_hide() {
    let mut ui = Harness::new(Visibility::Hover);
    let _ = ui.point(INSIDE);
    let _ = ui.redraw(16);
    let _ = ui.point(OUTSIDE);
    let _ = ui.redraw(100);
    let _ = ui.point(INSIDE);
    assert_eq!(
        ui.redraw(200),
        RedrawRequest::Wait,
        "back over it, nothing is pending"
    );
}

#[test]
fn scroll_shows_on_wheel_and_fades_after_the_scroll_ends() {
    let mut ui = Harness::new(Visibility::Scroll);
    let _ = ui.point(INSIDE);
    assert_eq!(
        ui.redraw(16),
        RedrawRequest::Wait,
        "pointing at the content does not show it"
    );

    let _ = ui.wheel();
    let hide = ui.at(1_000) + SCROLL_END + HIDE_DELAY;
    assert_eq!(
        ui.redraw(1_000),
        RedrawRequest::At(hide),
        "a scroll shows it"
    );
    let _ = ui.wheel();
    let hide = ui.at(1_200) + SCROLL_END + HIDE_DELAY;
    assert_eq!(
        ui.redraw(1_200),
        RedrawRequest::At(hide),
        "scrolling again extends it"
    );
    ui.hide(1_200 + millis(SCROLL_END + HIDE_DELAY));
}

#[test]
fn scroll_holds_while_the_pointer_is_on_the_scrollbar() {
    let mut ui = Harness::new(Visibility::Scroll);
    let _ = ui.point(INSIDE);
    let _ = ui.wheel();
    let _ = ui.redraw(16);
    let _ = ui.point(ON_BAR);
    assert_eq!(
        ui.redraw(100),
        RedrawRequest::Wait,
        "on the scrollbar, it stays"
    );
    assert_eq!(ui.redraw(5_000), RedrawRequest::Wait);
    let _ = ui.point(INSIDE);
    assert_eq!(
        ui.redraw(6_000),
        RedrawRequest::At(ui.at(6_000) + HIDE_DELAY),
        "leaving the scrollbar starts the hide delay"
    );
    ui.hide(6_000 + millis(HIDE_DELAY));
}

#[test]
fn always_and_auto_never_schedule_a_hide() {
    for visibility in [Visibility::Always, Visibility::Auto] {
        let mut ui = Harness::new(visibility);
        let _ = ui.point(INSIDE);
        let _ = ui.wheel();
        let _ = ui.redraw(16);
        let _ = ui.point(OUTSIDE);
        assert_eq!(ui.redraw(32), RedrawRequest::Wait, "{visibility:?}");
        assert_eq!(ui.redraw(5_000), RedrawRequest::Wait, "{visibility:?}");
    }
}

#[test]
fn a_custom_hide_delay_is_used() {
    let delay = Duration::from_millis(1_500);
    let mut ui = Harness::with_delay(Visibility::Hover, delay);
    let _ = ui.point(INSIDE);
    let _ = ui.redraw(16);
    let _ = ui.point(OUTSIDE);
    assert_eq!(ui.redraw(100), RedrawRequest::At(ui.at(100) + delay));
    ui.hide(100 + millis(delay));
}

#[test]
fn hover_fades_out_when_asked() {
    let mut ui = Harness::fading(Visibility::Hover);
    let _ = ui.point(INSIDE);
    let _ = ui.redraw(16);
    let _ = ui.point(OUTSIDE);
    assert_eq!(ui.redraw(100), RedrawRequest::At(ui.at(100) + HIDE_DELAY));
    ui.hide(100 + millis(HIDE_DELAY));
}

#[test]
fn scroll_fades_out_when_asked() {
    let mut ui = Harness::fading(Visibility::Scroll);
    let _ = ui.point(INSIDE);
    let _ = ui.wheel();
    let hide = ui.at(16) + SCROLL_END + HIDE_DELAY;
    assert_eq!(ui.redraw(16), RedrawRequest::At(hide));
    ui.hide(16 + millis(SCROLL_END + HIDE_DELAY));
}
