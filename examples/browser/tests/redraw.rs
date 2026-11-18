//! The frames the text fields ask for: a focused field left alone draws only
//! to blink its caret, an edit draws at once, and the caret stays on while
//! keys come.
use std::sync::Arc;

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::mouse::{self, Button};
use iced::{Element, Event, Point, Rectangle, Size};
use iced_test::core::InputMethod;
use iced_test::core::clipboard;
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
use icedcn::widget::{Platform, code_block, container, input, textarea};

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
            container(code_block(
                code_block::content("")
                    .language("rust")
                    .default_value(String::new())
                    .on_input(|s| s),
            ))
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

/// A text field drawn by a real renderer, keyed on the real clock, to see
/// whether its caret shows.
struct Blinking {
    ui: UserInterface<'static, String, Theme, Renderer>,
    renderer: Renderer,
    clipboard: Option<String>,
    caret: Option<Rectangle>,
}

impl Blinking {
    fn new(view: Element<'static, String, Theme>) -> Self {
        Platform::set(Some(Platform::Linux));
        let mut renderer = block_on(Renderer::new(Default::default(), Some("tiny-skia")))
            .expect("headless renderer");
        let ui = UserInterface::build(
            view,
            Size::new(800., 600.),
            user_interface::Cache::default(),
            &mut renderer,
        );
        let mut field = Self {
            ui,
            renderer,
            clipboard: Some(" ".into()),
            caret: None,
        };
        let mut window = Window {
            ui: std::mem::replace(
                &mut field.ui,
                UserInterface::build(
                    Element::from(input::<String>("")),
                    Size::new(800., 600.),
                    user_interface::Cache::default(),
                    &mut field.renderer,
                ),
            ),
            renderer: block_on(Renderer::new(Default::default(), None)).expect("renderer"),
            now: Instant::now(),
            request: RedrawRequest::Wait,
        };
        let at = window.focus();
        let _ = at;
        field.ui = window.ui;
        field
    }

    fn send(&mut self, events: Vec<Event>) {
        let mut events = events;
        while !events.is_empty() {
            let mut bus = Bus::new();
            let (state, _) = self.ui.update(
                &window::Headless,
                &Waker::noop(),
                &events,
                mouse::Cursor::Available(Point::new(790., 590.)),
                &mut self.renderer,
                &mut bus,
            );
            events.clear();
            if let user_interface::State::Updated {
                clipboard,
                input_method,
                ..
            } = state
            {
                if let InputMethod::Enabled { cursor, .. } = input_method {
                    self.caret = Some(cursor);
                }
                for _ in clipboard.reads {
                    events.push(Event::Clipboard(clipboard::Event::Read(Ok(Arc::new(
                        clipboard::Content::Text(self.clipboard.clone().unwrap_or_default()),
                    )))));
                }
            }
        }
    }

    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) {
        let key = key.into();
        let with = |event: Event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                text,
                repeat,
                ..
            }) => Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                modifiers,
                text,
                repeat,
            }),
            event => event,
        };
        self.send(vec![
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)),
            with(simulator::press_key(key.clone(), None)),
            simulator::release_key(key),
            Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::empty())),
        ]);
    }

    /// Whether a frame drawn now shows the caret.
    fn shows_caret(&mut self) -> bool {
        self.send(vec![Event::Window(window::Event::RedrawRequested(
            Instant::now(),
        ))]);
        let theme = Theme::light();
        self.ui.draw(
            &mut self.renderer,
            &theme,
            &renderer::Style::default(),
            mouse::Cursor::Available(Point::new(790., 590.)),
        );
        let caret = self.caret.expect("the caret's place");
        let rgba = self
            .renderer
            .screenshot(Size::new(800, 600), 1., theme.neutral.base.into());
        let gray = |x: usize, y: usize| i32::from(rgba[(y * 800 + x) * 4 + 1]);
        let x = caret.x.round() as usize;
        let rows = caret.y.ceil() as usize + 2..(caret.y + caret.height) as usize - 2;
        let background = gray(x + 12, rows.start);
        let inked = rows
            .clone()
            .filter(|&y| {
                (x.saturating_sub(1)..=x + 1).any(|x| (gray(x, y) - background).abs() > 60)
            })
            .count();
        inked * 2 > rows.len()
    }
}

#[test]
fn the_caret_stays_on_while_keys_come_and_blinks_again_after_a_pause() {
    std::thread::scope(|scope| {
        for name in ["input", "textarea", "code block"] {
            let _ = scope.spawn(move || {
                let view: Element<'static, String, Theme> = match name {
                    "input" => input("").width(300).on_input(|s| s).into(),
                    "textarea" => container(textarea("").on_input(|s| s)).width(300).into(),
                    _ => container(code_block(
                        code_block::content("")
                            .default_value(String::new())
                            .on_input(|s| s),
                    ))
                    .width(300)
                    .into(),
                };
                let mut field = Blinking::new(view);
                let pause = || std::thread::sleep(Duration::from_millis(300));
                let ctrl = Modifiers::CTRL;
                let keys: [(&str, Box<dyn Fn(&mut Blinking)>); 8] = [
                    (
                        "typing",
                        Box::new(|f| f.send(simulator::typewrite(" ").collect())),
                    ),
                    (
                        "typing on",
                        Box::new(|f| f.send(simulator::typewrite(" ").collect())),
                    ),
                    (
                        "a delete",
                        Box::new(|f| f.key(Named::Backspace, Modifiers::empty())),
                    ),
                    (
                        "a motion",
                        Box::new(|f| f.key(Named::ArrowLeft, Modifiers::empty())),
                    ),
                    (
                        "a word motion",
                        Box::new(move |f| f.key(Named::ArrowRight, ctrl)),
                    ),
                    (
                        "a paste",
                        Box::new(move |f| f.key(Key::Character("v".into()), ctrl)),
                    ),
                    (
                        "an undo",
                        Box::new(move |f| f.key(Key::Character("z".into()), ctrl)),
                    ),
                    (
                        "typing again",
                        Box::new(|f| f.send(simulator::typewrite(" ").collect())),
                    ),
                ];
                for (step, press) in keys {
                    press(&mut field);
                    pause();
                    assert!(
                        field.shows_caret(),
                        "{name}: the caret shows 300 ms after {step}"
                    );
                }
                pause();
                assert!(
                    !field.shows_caret(),
                    "{name}: the caret blinks off after a pause"
                );
            });
        }
    });
}
