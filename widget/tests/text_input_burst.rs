//! A controlled input keeps every keystroke of a burst.
//!
//! The runtime hands a window all the events that arrived since the last
//! frame in one `UserInterface::update` call, and only then runs the
//! application's `update` and `view`. Each of these tests drives that same
//! cycle: one batch of events against one widget tree, then every published
//! message through the application, then a fresh `view` from the new state.
//!
//! A [`Component`] (a dialog, a popover, a select...) rebuilds its view as
//! soon as something inside it publishes, from the values the application
//! gave it when it was built: the input inside is a new widget, holding the
//! value from before the burst, while the rest of the burst arrives.
use iced_core::clipboard;
use iced_core::input_method;
use iced_core::keyboard::{self, key};
use iced_core::mouse;
use iced_core::renderer;
use iced_core::shell;
use iced_core::widget::{self, operation};
use iced_core::window;
use iced_core::{Element, Event, Rectangle, Size};
use iced_runtime::UserInterface;
use iced_runtime::user_interface;
use iced_test::simulator;
use iced_widget::{Component, Renderer, Widget, component};
use icedcn_core::Theme;
use icedcn_widget::{Slot, dialog, input, input_group, space};

use std::sync::Arc;

const INPUT: widget::Id = widget::Id::new("input");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Input(String),
    Open(bool),
}

/// Where the input sits in the view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Host {
    /// Directly in the view.
    Plain,
    /// Inside a [`Component`], which rebuilds its view from the value the
    /// application gave it as soon as the input publishes, before the
    /// application has seen the message.
    Component,
    /// In an open [`dialog`], inside an input group, as the command palette
    /// and the connection forms of an application hold it.
    Dialog,
}

fn field(value: &str) -> Slot<'_, Message> {
    input("")
        .id(INPUT)
        .value(value)
        .on_input(Message::Input)
        .into()
}

struct Wrapper<'a> {
    value: &'a str,
}

impl<'a> Component<'a, Message, Theme, Renderer> for Wrapper<'a> {
    type State = ();
    type Event = Message;

    fn update(&self, _state: &mut (), event: Message, _renderer: &Renderer) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &()) -> impl Widget<Message, Theme, Renderer> + 'a {
        field(self.value).compose()
    }
}

fn view(host: Host, value: &str) -> Element<'_, Message, Theme, Renderer> {
    match host {
        Host::Plain => field(value).compose(),
        Host::Component => component(Wrapper { value }).boxed(),
        Host::Dialog => dialog(
            |_| space().width(10).height(10).into(),
            move |_| input_group(input("").id(INPUT).value(value).on_input(Message::Input)).into(),
        )
        .open(true)
        .on_open_change(Message::Open)
        .into(),
    }
}

/// A tiny application runtime: the value it owns, how it accepts an edit,
/// and the widget cache kept between frames.
struct App {
    host: Host,
    value: String,
    accept: fn(&str, String) -> String,
    renderer: Renderer,
    cache: user_interface::Cache,
    /// Every message the application received, in order.
    received: Vec<String>,
}

impl App {
    fn new(host: Host, value: &str) -> Self {
        Self::with(host, value, |_, new| new)
    }

    fn with(host: Host, value: &str, accept: fn(&str, String) -> String) -> Self {
        let renderer = iced_runtime::futures::futures::executor::block_on(
            <Renderer as renderer::Headless>::new(renderer::Settings::default(), None),
        )
        .expect("create headless renderer");

        let mut app = Self {
            host,
            value: value.to_owned(),
            accept,
            renderer,
            cache: user_interface::Cache::default(),
            received: Vec::new(),
        };

        app.frame(|ui, renderer| {
            ui.operate(renderer, &mut operation::focusable::focus(INPUT));
            ui.operate(
                renderer,
                &mut operation::text_input::move_cursor_to_end(INPUT),
            );
        });

        app
    }

    /// Builds the view, runs `f` against it, keeps its cache.
    fn frame(
        &mut self,
        f: impl FnOnce(&mut UserInterface<'_, Message, Theme, Renderer>, &mut Renderer),
    ) {
        let cache = std::mem::take(&mut self.cache);
        let mut ui = UserInterface::build(
            view(self.host, &self.value),
            Size::new(400.0, 300.0),
            cache,
            &mut self.renderer,
        );

        f(&mut ui, &mut self.renderer);

        self.cache = ui.into_cache();
    }

    /// One frame of the runtime: every event against one widget tree, then
    /// every message through `update`, as `iced_winit` does.
    fn burst(&mut self, events: impl IntoIterator<Item = Event>) -> &str {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = shell::Bus::new();

        self.frame(|ui, renderer| {
            let _ = ui.update(
                &window::Headless,
                &shell::Waker::noop(),
                &events,
                mouse::Cursor::Unavailable,
                renderer,
                &mut messages,
            );
        });

        for (message, _receipt) in messages.drain() {
            let Message::Input(value) = message else {
                continue;
            };
            self.received.push(value.clone());
            self.value = (self.accept)(&self.value, value);
        }

        // The next frame: the view reflects the new state.
        self.frame(|_, _| {});

        &self.value
    }

    /// What the input shows.
    fn shown(&mut self) -> String {
        let mut shown = Shown(None);

        self.frame(|ui, renderer| ui.operate(renderer, &mut shown));

        shown.0.expect("the input is in the view")
    }
}

/// Reads what the input shows.
struct Shown(Option<String>);

impl widget::Operation for Shown {
    fn text_input(
        &mut self,
        id: Option<&widget::Id>,
        _bounds: Rectangle,
        state: &mut dyn operation::TextInput,
    ) {
        if id == Some(&INPUT) {
            self.0 = Some(state.text().into_owned());
        }
    }

    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn widget::Operation)) {
        operate(self);
    }
}

const HOSTS: [Host; 3] = [Host::Plain, Host::Component, Host::Dialog];

fn key(named: key::Named) -> Event {
    simulator::press_key(keyboard::Key::Named(named), None)
}

/// Command-V, then the clipboard's reply.
fn paste(text: &str) -> [Event; 2] {
    let v = keyboard::Key::Character("v".into());

    [
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: v.clone(),
            modified_key: v,
            physical_key: keyboard::key::Physical::Code(keyboard::key::Code::KeyV),
            location: keyboard::Location::Standard,
            modifiers: keyboard::Modifiers::COMMAND,
            repeat: false,
            text: None,
        }),
        Event::Clipboard(clipboard::Event::Read(Ok(Arc::new(
            clipboard::Content::Text(text.to_owned()),
        )))),
    ]
}

/// What `f` gives for every host, to compare with one expected answer.
fn each<T>(f: impl Fn(Host) -> T) -> Vec<(Host, T)> {
    HOSTS.into_iter().map(|host| (host, f(host))).collect()
}

fn all<T: Clone>(expected: T) -> Vec<(Host, T)> {
    each(|_| expected.clone())
}

#[test]
fn a_burst_of_keystrokes_reaches_the_application() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");
        let value = app.burst(simulator::typewrite("bcd")).to_owned();

        (value, app.received.clone(), app.shown())
    });

    assert_eq!(
        outcome,
        all((
            String::from("abcd"),
            vec![
                String::from("ab"),
                String::from("abc"),
                String::from("abcd")
            ],
            String::from("abcd"),
        ))
    );
}

#[test]
fn a_burst_with_backspace_and_cursor_moves_edits_its_own_result() {
    let outcome = each(|host| {
        let mut app = App::new(host, "abc");

        let events = [key(key::Named::Backspace), key(key::Named::Backspace)]
            .into_iter()
            .chain(simulator::typewrite("xy"))
            .chain([key(key::Named::ArrowLeft)])
            .chain(simulator::typewrite("z"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("axzy")));
}

#[test]
fn a_burst_with_a_paste_keeps_what_came_before_and_after() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");

        let events = simulator::typewrite("b")
            .chain(paste("XY"))
            .chain(simulator::typewrite("c"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("abXYc")));
}

#[test]
fn a_burst_with_an_input_method_commit_keeps_it() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");

        let events = simulator::typewrite("b")
            .chain([Event::InputMethod(input_method::Event::Commit(
                String::from("é"),
            ))])
            .chain(simulator::typewrite("c"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("abéc")));
}

#[test]
fn a_burst_replaces_a_selection_once() {
    let outcome = each(|host| {
        let mut app = App::new(host, "old");

        app.frame(|ui, renderer| {
            ui.operate(renderer, &mut operation::text_input::select_all(INPUT));
        });

        app.burst(simulator::typewrite("new")).to_owned()
    });

    assert_eq!(outcome, all(String::from("new")));
}

#[test]
fn an_application_that_rewrites_the_value_wins_after_the_burst() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |_, value| value.to_uppercase());

        let first = app.burst(simulator::typewrite("bc")).to_owned();
        let shown = app.shown();
        // The next burst edits the rewritten value.
        let second = app.burst(simulator::typewrite("d")).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((
            String::from("ABC"),
            String::from("ABC"),
            String::from("ABCD")
        ))
    );
}

#[test]
fn an_application_that_limits_the_value_wins_after_the_burst() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |_, value| value.chars().take(3).collect());

        let first = app.burst(simulator::typewrite("bcde")).to_owned();
        let shown = app.shown();
        let second = app.burst([key(key::Named::Backspace)]).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((String::from("abc"), String::from("abc"), String::from("ab")))
    );
}

#[test]
fn an_application_that_rejects_the_edit_restores_its_value() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |old, _| old.to_owned());

        let first = app.burst(simulator::typewrite("bc")).to_owned();
        let shown = app.shown();
        let second = app.burst(simulator::typewrite("z")).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((String::from("a"), String::from("a"), String::from("a")))
    );
}
