//! The frames the text fields ask for: a focused field left alone draws only
//! to blink its caret, and an edit draws at once.
use iced::mouse::{self, Button};
use iced::{Element, Event, Point, Rectangle, Size};
use iced_test::core::renderer::{self, Headless};
use iced_test::core::shell::{Bus, Waker};
use iced_test::core::time::{Duration, Instant};
use iced_test::core::widget::operation::TextInput;
use iced_test::core::widget::{Id, Operation};
use iced_test::core::window::{self, RedrawRequest};
use iced_test::futures::futures::executor::block_on;
use iced_test::renderer::Renderer;
use iced_test::runtime::user_interface::{self, UserInterface};
use iced_test::simulator;
use icedcn::Theme;
use icedcn::widget::{code_block, container, input, textarea};

/// A user interface driven like the runtime drives a window: each frame is
/// scheduled from the redraw the frame before asked for.
struct Window {
    ui: UserInterface<'static, String, Theme, Renderer>,
    renderer: Renderer,
    now: Instant,
    request: RedrawRequest,
}

impl Window {
    fn new(view: impl Into<Element<'static, String, Theme>>) -> Self {
        let mut renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let ui = UserInterface::build(
            view.into(),
            Size::new(800., 600.),
            user_interface::Cache::default(),
            &mut renderer,
        );
        Self {
            ui,
            renderer,
            now: Instant::now(),
            request: RedrawRequest::Wait,
        }
    }

    /// Send `events` with the pointer at `cursor`; returns the redraw they
    /// ask for.
    fn send(&mut self, events: &[Event], cursor: Point) -> RedrawRequest {
        let mut bus = Bus::new();
        let (state, _) = self.ui.update(
            &window::Headless,
            &Waker::noop(),
            events,
            mouse::Cursor::Available(cursor),
            &mut self.renderer,
            &mut bus,
        );
        match state {
            user_interface::State::Updated { redraw_request, .. } => redraw_request,
            user_interface::State::Outdated => RedrawRequest::NextFrame,
        }
    }

    fn frame(&mut self, cursor: Point) {
        let now = self.now;
        self.request = self.send(
            &[Event::Window(window::Event::RedrawRequested(now))],
            cursor,
        );
        self.ui.draw(
            &mut self.renderer,
            &Theme::light(),
            &renderer::Style::default(),
            mouse::Cursor::Available(cursor),
        );
    }

    /// The frames drawn over `span` left alone, a requested next frame
    /// coming 16 milliseconds later.
    fn idle(&mut self, span: Duration, cursor: Point) -> usize {
        let end = self.now + span;
        let mut frames = 0;
        loop {
            self.now = match self.request {
                RedrawRequest::NextFrame => self.now + Duration::from_millis(16),
                RedrawRequest::At(at) => at.max(self.now),
                RedrawRequest::Wait => return frames,
            };
            if self.now > end {
                return frames;
            }
            frames += 1;
            self.frame(cursor);
        }
    }

    /// Focus the one text field with a click near its start, leaving the
    /// pointer away from it.
    fn focus(&mut self) -> Point {
        #[derive(Default)]
        struct Fields(Vec<Rectangle>);
        impl Operation for Fields {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn TextInput) {
                self.0.push(bounds);
            }
        }
        let mut fields = Fields::default();
        self.ui.operate(&self.renderer, &mut fields);
        let bounds = fields.0.pop().expect("a text field");
        let at = Point::new(bounds.x + 14., bounds.y + 10.);
        let away = Point::new(790., 590.);
        let _ = self.send(
            &[Event::Mouse(mouse::Event::CursorMoved { position: at })],
            at,
        );
        let _ = self.send(
            &[
                Event::Mouse(mouse::Event::ButtonPressed(Button::Left)),
                Event::Mouse(mouse::Event::ButtonReleased(Button::Left)),
            ],
            at,
        );
        let _ = self.send(
            &[Event::Mouse(mouse::Event::CursorMoved { position: away })],
            away,
        );
        self.request = RedrawRequest::NextFrame;
        away
    }
}

fn fields() -> [(&'static str, Element<'static, String, Theme>); 4] {
    [
        ("input", input("").width(300).into()),
        (
            "listened input",
            input("").width(300).on_input(|s| s).into(),
        ),
        ("textarea", textarea("").on_input(|s| s).into()),
        (
            "code block",
            container(code_block![
                code_block::content("")
                    .language("rust")
                    .default_value(String::new())
                    .on_input(|s| s)
            ])
            .width(320)
            .into(),
        ),
    ]
}

#[test]
fn a_focused_field_left_alone_draws_only_to_blink_its_caret() {
    for (name, view) in fields() {
        let mut window = Window::new(view);
        let away = window.focus();
        // The focus ring eases in over its first frames.
        let _ = window.idle(Duration::from_millis(500), away);
        let frames = window.idle(Duration::from_secs(5), away);
        // The caret turns on and off every 500 milliseconds.
        assert!(
            (8..=11).contains(&frames),
            "{name} drew {frames} frames in 5 idle seconds"
        );
    }
}

#[test]
fn an_edit_draws_at_once() {
    for (name, view) in fields() {
        let mut window = Window::new(view);
        let away = window.focus();
        let _ = window.idle(Duration::from_millis(500), away);
        let typed: Vec<Event> = simulator::typewrite("a").collect();
        assert_eq!(
            window.send(&typed, away),
            RedrawRequest::NextFrame,
            "{name} draws the typed character"
        );
    }
}
